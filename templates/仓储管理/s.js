daysMap = new Map()
editor = null; // RTF编辑器
isLoading = false; cancelUploadFunc = undefined; app={entershoulddefault:false};
前缀str = `<html lang='zh'>
<head>
<meta charset="UTF-8">
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap@4.5.3/dist/css/bootstrap.min.css"
  integrity="sha256-93wNFzm2GO3EoByj9rKZCwGjAJAwr0nujPaOgwUt8ZQ=" crossorigin="anonymous">
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap-vue@2.21.2/dist/bootstrap-vue.min.css"
  integrity="sha256-cAyk5KZc7P6j7j/uL7MOoN4PRsZYp+BN9yo03Y6Qk38=" crossorigin="anonymous">
<style>*{font-family:'Sarasa Term SC'}body{white-space:pre-wrap}</style>
</head>
<body><div id="app"><!-- vue instance -->`;

中缀str = `<!-- vue instance end --></div></body>
<sc`+`ript src="https://cdn.jsdelivr.net/npm/vue@2.6.14/dist/vue.min.js"
  integrity="sha256-kXTEJcRFN330VirZFl6gj9+UM6gIKW195fYZeR3xDhc=" crossorigin="anonymous"></sc`+`ript>
<sc`+`ript src="https://cdn.jsdelivr.net/npm/bootstrap-vue@2.21.2/dist/bootstrap-vue.min.js"
  integrity="sha256-duDNIhfVrUK7HB/57nPLxN/j92aM3rhTUFzVI/H5ex8=" crossorigin="anonymous"></sc`+`ript>
<sc`+`ript>`;

后缀str = `</sc`+`ript>
</html>`;

上次刷新的时间 = new Date('1995-12-17T03:24:00');
本次input的时间 = new Date('1995-12-17T03:24:00');

var getProgText = function (selector) {
  var el = document.querySelector(selector);
  var childs = el.childNodes;
  for(var i = childs.length - 1; i >= 0; i--) {
    if(childs[i].tagName=="BR"){el.removeChild(childs[i]);}
  }
  return el.innerText
};
var cumulativeOffset = function (element) {
  var top = 0, left = 0;
  do {
    top += element.offsetTop || 0;
    left += element.offsetLeft || 0;
    element = element.offsetParent;
  } while (element);
  return {
    top: top,
    left: left
  };
};
function copyStringToClipboard (str) {
  try { // Remove temporary element
    document.body.removeChild(textarea_copy_el);
  } catch (error) {console.error(error);}
  // Create new element
  textarea_copy_el = document.createElement('textarea');
  // Set value (string to be copied)
  textarea_copy_el.value = str;
  // Set non-editable to avoid focus and move outside of view
  textarea_copy_el.setAttribute('readonly', '');
  textarea_copy_el.style.position="absolute"
  textarea_copy_el.style.left="-9999px"
  document.body.appendChild(textarea_copy_el);
  // Select text inside element
  textarea_copy_el.select();
  // Copy text to clipboard
  document.execCommand('copy');
}
var app = new Vue({
  el: '#app',
  data: {
    isLoggedIn: "", weekDayMap:["周日","周一","周二","周三","周四","周五","周六"], allowTypeSelection:true, finishedTimeAsc:false,
    fullitems: [],
    warehouses: [""],
    search_result: [],
    busy_connecting_backend: false,
    isQueryingBackend: false,
    entershoulddefault: false, 
    ConfirmText:'', ConfirmResult:null, ConfirmPromiseResolve:null,
    page_history: 1,
    query_txt: '',
    resnumber: 888,
    edittgtitem: {type:"default",名称:"垃圾堆",描述:"垃圾堆",数量:1,内容:[]},
    edittgtfile: ['','','','attachment'], fileUploaded: false, uploadPercentCompleted: 0,
    focused_elem_stack: [],
    show_search_result_mode: false,
    currentDraggedElem:null,
    currentDragTarget:null,
    consoleCmd:"",
    backupList:[], newBackupName:"",
    showSidebar:false,showBackupSidebar:false,
    previewImgURL:"",
    loadedJsLibs:{hljs:false,kindeditor:false}, tmpProgType:"clip", tmpProgText:"", tmpProgTextRealtime:"", tmpProgText2:"", tmpProgText2Realtime:"", 
    tmpProgLang:"Language-python", tmpProgLangUserInput:"go",
    editorText: '',
    batchSentences: '', copyPaste1:''
  },
  mounted() {
    window.history.pushState({page: 1}, "", "");
    if(this.CurWarehouse.slice(0,11)=='初始化URLBase-'){this.CurWarehouse=this.CurWarehouse.slice(11);this.getitems([]);} //如果有URL预设，那就要这一家仓库的。
    else{this.CurWarehouse='当前focus数据结构';this.getitems([]);}
    this.getwarehouses();
  },
  methods: {
    dragstartF: function (e, item) {
      document.getElementById("dragCursor").style.display='block';
      this.currentDraggedElem = item;
    },
    dragenterF: function (e, item) {
      e.stopPropagation();
      curelem=e.target;
      while ( (!curelem.classList.contains("bdispl") )&&(!curelem.classList.contains("bbtmbtn") )) {
        curelem = curelem.parentElement;
      }
      var cur = document.getElementById("dragCursor");
      curelem.parentElement.insertBefore(cur, curelem);
      this.currentDragTarget=item.ind.concat();
    },
    dragleaveF: function (e, item) {
    },
    dragenterFLast: function (e, parentitem) {
      e.stopPropagation();
      curelem=e.target;
      while ( (!curelem.classList.contains("bdispl") )&&(!curelem.classList.contains("bbtmbtn") )) {
        curelem = curelem.parentElement;
      }
      var cur = document.getElementById("dragCursor");
      curelem.parentElement.insertBefore(cur, curelem);
      if(!parentitem.ind){this.currentDragTarget=[this.fullitems.length].concat()}
      else{this.currentDragTarget=parentitem.ind.concat().concat([parentitem.内容.length]);}
    },
    dragleaveFLast: function (e, parentitem) {
    },
    dropF: function (e, item) {
      var cur = document.getElementById("dragCursor");
      document.getElementById("dragCursorMark").parentElement.insertBefore(cur, document.getElementById("dragCursorMark"));
      cur.style.display='none'; // 删除光标。
      // 拖拽的最终位置，应当在dragenter时搞定，此处直接使用。
      // 此外，dragenter也需要判断是否拖入了自己的子树，防止这种情况发生。
      console.log(item.ind) // source
      // console.log(this.currentDraggedElem)
      console.log(this.currentDragTarget) // target
      // 对于source没有任何限制。
      // 但需要判断target是否为非法值；以及，移动之后需要清除前端的子树、reload source和target的各层，以防indice与后端冲突。
      if(this.currentDragTarget.length>=item.ind.length){
        issub = true;
        for (let i = 0; i < item.ind.length; i++) {
          if(this.currentDragTarget[i]!=item.ind[i]){issub = false}
        }
        if(issub&&this.currentDragTarget.length>item.ind.length){window.alert("不能将物体移动到其内部的子节点中！当前移动操作已取消，请重试。");return;}
        issub = true;
        for (let i = 0; i < item.ind.length-1; i++) {
          if(this.currentDragTarget[i]!=item.ind[i]){issub = false}
        }
        if(issub&&this.currentDragTarget[item.ind.length-1] > item.ind[item.ind.length-1]){
          this.currentDragTarget[item.ind.length-1] --;
        }
      }
      console.log("newtarget",this.currentDragTarget)
      // 目前，展示的东西似乎移动到了正确的位置，但ind仍然不正确。为免复杂的递归代码，此处应向后端传递信息，再从后端重新索取新信息。
      axios.post('/storageManager/move-items', {src:item.ind,tgt:this.currentDragTarget,warehs:this.CurWarehouse,baseInd:this.baseInd}).then(function (response) {
        resptext = response.data; resvals = eval(resptext);
        // 首先移除source。
        target = app.fullitems;
        for (const i of item.ind.slice(0,-1)) {
          target = target[i].内容
        }
        被移动物拷贝 = JSON.parse(JSON.stringify(target[item.ind[item.ind.length-1]]));
        app.$delete(target,item.ind[item.ind.length-1]);
        // ↑上面的代码没啥意义，↓反正下边也要更新……
        target = app.fullitems;
        level = item.ind.slice(0,-1);
        if(level.length==0){app.fullitems= resvals[0];return}
        for (const i of level.slice(0,-1)) {
          target = target[i].内容
        }
        target[level[level.length-1]].内容 = resvals[0];
        // 然后，将拷贝粘贴到新位置。
        try {
          target = app.fullitems;for (const i of app.currentDragTarget.slice(0,-1)) {target = target[i].内容};
          target.splice(app.currentDragTarget[app.currentDragTarget.length-1],0, 被移动物拷贝);
          //
          target = app.fullitems;
          level = app.currentDragTarget.slice(0,-1);
          if(level.length==0){app.fullitems= resvals[1];return}
          for (const i of level.slice(0,-1)) {
            target = target[i].内容
          }
          target[level[level.length-1]].内容 = resvals[1];
        } catch (error) {;} // 这里catch是由于，本段操作确实可能在正常情况下失败，例如将同一树下层级2的节点移到其它层级2的节点之中，此时新位置经过上面的代码已经不存在。
        app.$forceUpdate();
      }).catch(function (error) {
        console.log(error);window.alert("获取后端信息出错，为了保证数据结构不受损害，请刷新页面！！")
      })
      app.$forceUpdate();
    },
    bdisplhover: function (event) {
      event.stopPropagation();
      tmpstack = []; curelem=event.target;
      while (curelem.parentElement.parentElement.classList.contains("bdispl")) {
        curelem = curelem.parentElement.parentElement;
        tmpstack.push(curelem)
      }
      tmpstack.reverse();
      if(this.focused_elem_stack.length > tmpstack.length){return}
      try {
        this.focused_elem_stack.forEach(element => {
          element.className = element.className.replace(' TheFocusedItem','')
        });
      } catch (error) {}
      this.focused_elem_stack = tmpstack;
      // ↑重算focus_elem_stack。
      this.focused_elem_stack.forEach(element => {
        element.className = element.className.replace(' TheFocusedItem','')
      });
      this.focused_elem_stack.push(event.target);
      event.target.className = event.target.className + ' TheFocusedItem';
      return;
    },
    bdisplleave: function (event) {
      var element = this.focused_elem_stack.pop();
      if(element){
        element.className = element.className.replace(' TheFocusedItem','')
      }
      var tmpe = this.focused_elem_stack.pop();
      if(tmpe){
      this.focused_elem_stack.push(tmpe);
      tmpe.className = tmpe.className + ' TheFocusedItem';}
      event.stopPropagation();
      return;
    },
    toggleExpand:  function (item) {item.expandDesc=!(item.expandDesc)},
    login: function () {
      if (this.signup_username.length < 2) { window.alert("用户名长度不得小于2位，请重新输入！"); return; }
      if (this.signup_password.length < 5) { window.alert("密码长度不得小于5位，请重新输入！"); return; }
      console.log("log in.");
      var hashObj = new jsSHA("SHA-256", "TEXT", { numRounds: 1 });
      hashObj.update(this.signup_username + this.signup_password + "salter2333");
      var hash = hashObj.getHash("HEX");
      console.log('generated login auth:', hash, 'connecting to backend...');
      this.busy_connecting_backend = true;
      axios.post('/storageManager/log-in', {
        username: this.signup_username,
        auth: hash,
      }).then(function (response) {
        // console.log(response);
        // console.log(response.data);
        login_state = response.data;
        app.busy_connecting_backend = false;
        if (login_state != 'ok') {
          if (login_state == 'already_logged_in') { window.alert("当前用户已经登录。出现此提示应为前端错误，请您刷新网页。"); return; }
          if (login_state == 'nosignup') { window.alert("当前用户尚未注册。请您先注册哟~"); return; }
          window.alert("登录失败，应是用户名或密码错误。请检查输入~\n未注册需先注册。"); return;
        }
        app.$bvModal.hide('bv-modal-login');
        app.isLoggedIn = true;
        window.alert("登录成功！");
      }).catch(function (error) {
        console.log(error);
      })
    },
    logout: function () {
      console.log("log out.");
      axios.post('/storageManager/log-out', {}).then(function (response) {
        app.isLoggedIn = false;
        window.alert("退出登录成功！");
      }).catch(function (error) {
        console.log(error);
      })
    },
    getitems: function (level) {
      axios.post('/storageManager/get-items', {level:level,warehs:this.CurWarehouse,baseInd:this.baseInd}).then(function (response) {
        resptext = response.data;
        if(level.length==0){app.fullitems= eval(resptext);if(app.isTimeManagement){app.TimeSortAsc();}return}
        target = app.fullitems;
        for (const i of level.slice(0,-1)) {
          target = target[i].内容
        }
        target[level[level.length-1]].内容 = eval(resptext);
        console.log(target);
        app.$forceUpdate();
      }).catch(function (error) {
        console.log(error);window.alert("您可能未登录哟~或获取后端信息出错，为了保证数据结构不受损害，请刷新页面！！")
      })
    },
    prepareBackupSidebar: function () {
      this.showBackupSidebar=true;
      axios.post('/storageManager/get-backup-list', {}).then(function (response) {
        resp=eval(response.data);
        app.backupList=resp;
      }).catch(function (error) {
        console.log(error);window.alert("恢复出错，请尽量不刷新页面、然后重试！！")
      })
    },
    restoreBackup: function (backup) {
      if(!window.confirm("！！！此操作不可撤销！！！请确认操作。此外，您的操作将解压全部用户的备份、但只更新您的数据，这是程序的bug，如果想要节省恢复成本，需要程序员修改备份逻辑！")){return;}
      axios.post('/storageManager/restore-backup', {file:backup}).then(function (response) {
        window.alert("已完成备份，请检查！");
        app.showBackupSidebar=false;app.showSidebar=false;
      })
    },
    removeBackup: function (backup) {
      if(!window.confirm("！！！此操作不可撤销！！！请确认操作。")){return;}
      axios.post('/storageManager/remove-backup', {file:backup}).then(function (response) {
        window.alert("已删除备份，请检查！");
        axios.post('/storageManager/get-backup-list', {}).then(function (response) {
          resp=eval(response.data);app.backupList=resp;
        })
      })
    },
    createNewBackup: function () {
      if(!window.confirm("您确定要以【"+this.newBackupName+"】为名称创建备份吗~ （多用户逻辑不完善，请即联系程序员修改）")){return;}
      axios.post('/storageManager/create-backup', {name:this.newBackupName}).then(function (response) {
        window.alert("已完成备份，请检查！");
        app.showBackupSidebar=false;app.showSidebar=false;
      })
    },
    clearTemp: function () {
      axios.post('/storageManager/clear-temp', {action:"query"}).then(function (response) {
        resp=eval(response.data);
      if(!window.confirm("当前temp文件夹中共有"+resp+"个文件。请确认操作。")){return;}
      axios.post('/storageManager/clear-temp', {action:"clean"}).then(function (response) {
        window.alert("已清理temp文件夹！");
      })
      })
    },
    getwarehouses: function (level) {
      axios.post('/storageManager/get-warehouses', {}).then(function (response) {
        resp=eval(response.data);
        app.warehouses=resp.warehouses;
        if(app.CurWarehouse!='当前focus数据结构'){}
        else{app.CurWarehouse=resp.CurWarehouse;}
      })
    },
    executeCmdCommand: function () {
      tgtind = this.warehouses.indexOf(this.consoleCmd);
      if(tgtind==-1){window.alert("定位库失败，请确认库名称是否正确！");return;}
      this.warehouseFocus(tgtind)
    },
    rmWarehouse: function (ind) {
      if(!window.confirm("！！！此操作不可撤销！！！请确认操作")){return;}
      axios.post('/storageManager/remove-warehouse', {ind:ind}).then(function (response) {
        window.alert("删除成功");app.getwarehouses();
      })
    },
    warehouseFocus: function (ind) {
      axios.post('/storageManager/focus-warehouse', {ind:ind}).then(function (response) {
        app.CurWarehouse = app.warehouses[ind];app.showSidebar=false;
        app.$bvToast.toast("更改 warehouse focus 导航成功！", {title: '导航成功！',
          autoHideDelay: 3000,appendToast: true,variant: "primary",toaster: 'b-toaster-bottom-right'})
        app.baseInd=[];app.upperDirInfo=null;
        app.isTimeManagement = (app.CurWarehouse=='事件管理ToDo');
        app.getitems([]);app.show_search_result_mode=false;
      })
    },
    addWarehouse: function () {
      newname = "";
      while (newname=='' || /[.:/\\ ~><?]/g.test(newname) || this.warehouses.indexOf(newname)!=-1) {
        newname = prompt("请输入新库的名称（如果重复出现提示框，说明之前输入的包含非法字符）：")
      }
      axios.post('/storageManager/add-warehouse', {name:newname}).then(function (response) {
        window.alert("添加成功");app.getwarehouses();
      })
    },
    renameWarehouse: function (ind) {
      newname = "";
      while (newname=='' || /[.:/\\ ~><?]/g.test(newname) || this.warehouses.indexOf(newname)!=-1) {
        newname = prompt("请输入这个库的新名称（如果重复出现提示框，说明之前输入的包含非法字符，或命名与已有库重复）：")
      }
      axios.post('/storageManager/rename-warehouse', {ind:ind,name:newname}).then(function (response) {
        window.alert("改名成功");app.getwarehouses();
      })
    },
    moveUpWarehouse: function (ind) {axios.post('/storageManager/moveup-warehouse', {ind:ind}).then(function (response) {app.getwarehouses();})},
    moveDownWarehouse: function (ind) {axios.post('/storageManager/movedown-warehouse', {ind:ind}).then(function (response) {app.getwarehouses();})},
    recursivelyGetItems: function (level) {
      this.$bvToast.toast(`请注意不要急于操作。本次导航预计耗时：`+(0.6+level.length*0.2)+"秒。", {
        title: '⚠️正在导航，请注意不要急于操作！',
        autoHideDelay: 4000,
        appendToast: true,
        variant: "danger",
        toaster: 'b-toaster-bottom-right'
      })
      for (let i = 0; i < level.length; i++) {setTimeout(() => {app.getitems(level.slice(0,i+1))}, i*200) }
      setTimeout(() => {
        if(level.length==0){return}
        target = app.fullitems;
        for (const i of level.slice(0,-1)) {
          target = target[i].内容
        }
        console.log(target)
        target[level[level.length-1]].isfocus=true;
        app.show_search_result_mode = false;
        app.$forceUpdate();
        setTimeout(() => {
          window.scrollTo(0, cumulativeOffset(document.getElementById("resultScrollMark")).top-160);
          app.$forceUpdate();
          setTimeout(() => {
            delete target[level[level.length-1]].isfocus;
            app.$forceUpdate();
          }, 1500);
        }, 200);
      }, 400+level.length*200);
      return (0.6+level.length*0.2);
    },
    deleteitems: function (level) {
      if(!window.confirm("！warning！\n请确认删除操作，因为删除操作将会移除该项所包含的所有内容，包括文件、rtf、代码等等。是否确认删除该项？？？")){return}
      axios.post('/storageManager/delete-items', {level:level,warehs:this.CurWarehouse,baseInd:this.baseInd}).then(function (response) {
        resptext = response.data;
        if(level.length==0){app.fullitems=[];return}
        target = app.fullitems;
        for (const i of level.slice(0,-1)) {
          target = target[i].内容
        }
        app.$delete(target,level[level.length-1]);app.$forceUpdate();
        if(level.length==1){targetnewind=eval(resptext);for (let index = 0; index < targetnewind.length; index++) { app.fullitems[index].ind = targetnewind[index].ind; }return}
        target = app.fullitems;
        for (const i of level.slice(0,-1)) {
          target = target[i].内容
        }
        targetnewind = eval(resptext);
        for (let index = 0; index < targetnewind.length; index++) { target[index].ind = targetnewind[index].ind; }
        app.$bvToast.toast(`已删除~`, {title: '此操作不可撤回，若想撤销需要重新输入相关信息。', autoHideDelay: 2500, appendToast: true, variant: "danger", solid: true, toaster: 'b-toaster-bottom-right'})
      }).catch(function (error) {
        console.log(error);
      })
    },
    additems: function (level) {
      target = app.fullitems;for (const i of level) {target = target[i].内容};
      // target = target[level[level.length-1]];
      defaultstuff = {ind:level.concat([target.length]),type:"default",名称:"新建物品[默认值]",描述:"请输入该物品的描述！",词向量描述:["[<新物品>]"],句向量描述:[],数量:1,内容:[],files:[]};
      target.push(defaultstuff);
      axios.post('/storageManager/add-items', {level:level,stuff:defaultstuff,warehs:this.CurWarehouse,baseInd:this.baseInd}).then(function (response) {
        app.$bvToast.toast(`已添加新物品~请编辑物品属性哟。`, {title: '已添加新物品~', autoHideDelay: 2500, appendToast: true, variant: "info", solid: true, toaster: 'b-toaster-bottom-right'});
        var x = new Date();
        currentTimeZoneOffsetInHours = - x.getTimezoneOffset() / 60;
        var day = new Date(); day.setHours(day.getHours() + currentTimeZoneOffsetInHours); var str = day.toISOString();
        app.edittgtitem.modify = app.edittgtitem.setup = str.slice(0,10)+' '+str.slice(11,19);
        app.edittarget(defaultstuff.ind)
      }).catch(function (error) {console.log(error);})
      delete defaultstuff.内容
    },
    edittarget: function (level) {
      console.log("edittarget",level)
      if(level.length==0){this.edittgtitem=this.fullitems;return}
      target = app.fullitems;
      for (const i of level.slice(0,-1)) {
        target = target[i].内容
      }
      this.edittgtitem=target[level[level.length-1]];
      var it = this.edittgtitem;
      var x = new Date();
      currentTimeZoneOffsetInHours = - x.getTimezoneOffset() / 60;
      var day = new Date(); day.setHours(day.getHours() + currentTimeZoneOffsetInHours); var str = day.toISOString();
      this.allowTypeSelection=true;
      if(this.isTimeManagement){if(level.length<=1){Vue.set(it,"type","事件Event");this.allowTypeSelection=false}}
      if(!it.截止日期){Vue.set(it,"截止日期",str.slice(0,10))};
      if(!it.截止时间){Vue.set(it,"截止时间",'17:00')};
      if(!it.重复数){Vue.set(it,"重复数",0)};
      if(!it.重复间隔){Vue.set(it,"重复间隔",'天')};
      if(!it.已完成){Vue.set(it,"已完成",'未完成')};
      if(!it.数量){Vue.set(it,"数量",1)};
      this.$bvModal.show('edittargetmodal')
    },
    saveedittgt: function () {
      var a = document.getElementById("edittgtitem词向量描述");
      var vecarr = [];for (let index = 0; index < a.childNodes.length; index++) {
        const element = a.childNodes[index];
        vecarr.push(element.querySelector("input").value)
      }
      this.edittgtitem.词向量描述 = vecarr;
      // ↑workaround to vue v-for动态嵌套vmodel的bug。
      var a2 = document.getElementById("edittgtitem句向量描述");
      var vecarr2 = [];for (let index = 0; index < a2.childNodes.length; index++) {
        const element = a2.childNodes[index];
        vecarr2.push(element.querySelector("textarea").value)
      }
      this.edittgtitem.句向量描述 = vecarr2;
      // still workaround
      axios.post('/storageManager/save-items', {level:this.edittgtitem.ind,stuff:this.edittgtitem,warehs:this.CurWarehouse,baseInd:this.baseInd}).then(function (response) {
        resptext = response.data;
        console.log(resptext)
        let x = new Date();
        let currentTimeZoneOffsetInHours = - x.getTimezoneOffset() / 60;
        var day = new Date(); day.setHours(day.getHours() + currentTimeZoneOffsetInHours); var str = day.toISOString();
        app.edittgtitem.modify = str.slice(0,10)+' '+str.slice(11,19);
        if(resptext.startsWith("Exception")) {if(window.confirm("您输入的公式有误，或词语不支持。请重新输入词向量表示，否则搜索效果不佳！报错代码："+resptext)){app.$bvModal.show('edittargetmodal');}}
        app.$bvToast.toast('已保存位于程序数据结构['+app.edittgtitem.ind+']处的信息~', {title: `已保存~`, autoHideDelay: 2500, appendToast: true, variant: "info", solid: true, toaster: 'b-toaster-bottom-right'})
        if(app.edittgtitem.ind.length==1){if(app.isTimeManagement){app.TimeSortAsc();}} // 若todolist则自动排序
      }).catch(function (error) {console.log(error);})
      // 向后端保存信息
    },
    tgtitemDeleteFile: function (i) {
      axios.post('/storageManager/delete-file', {level:this.edittgtitem.ind,stuff:this.edittgtitem,rmid:i,file:this.edittgtitem.files[i],warehs:this.CurWarehouse,baseInd:this.baseInd}
          ).then(function (response) {app.edittgtitem.files.splice(i,1)})
    },
    initEditFileInfo: function () {
      this.fileUploaded=false;
      this.edittgtfile=['','','','attachment'];
      this.uploadPercentCompleted=0;
      this.$bvModal.show('editFileInfo');
    },
    uploadFile: function (file2) {
      var data = new FormData();
      console.log(file2)
      if(file2){var ext = file2.type.split('/').pop();
        if(file2.type=='text/plain'){ext='txt'}
        if(file2.type=='video/x-msvideo'){ext='avi'}
        if(file2.type=='application/octet-stream'){ext='bin'}
        if(file2.type=='application/msword'){ext='doc'}
        if(file2.type=='application/vnd.openxmlformats-officedocument.wordprocessingml.document'){ext='docx'}
        if(file2.type=='application/epub+zip'){ext='epub'}
        if(file2.type=='audio/mpeg'){ext='mp3'}
        if(file2.type=='application/vnd.ms-powerpoint'){ext='ppt'}
        if(file2.type=='application/vnd.openxmlformats-officedocument.presentationml.presentation'){ext='pptx'}
        if(file2.type=='application/vnd.rar'){ext='rar'}
        if(file2.type=='application/vnd.ms-excel'){ext='xls'}
        if(file2.type=='application/vnd.openxmlformats-officedocument.spreadsheetml.sheet'){ext='xlsx'}
        data.append('file', file2);
        let x = new Date();
        let currentTimeZoneOffsetInHours = - x.getTimezoneOffset() / 60;
        var day = new Date(); day.setHours(day.getHours() + currentTimeZoneOffsetInHours); var str = day.toISOString();
        this.edittgtfile[0] = 'Clipboard File '+str.slice(0,10)+' '+str.slice(11,19)+'.'+ext;
      }
      else{
        tgtfile = document.getElementById('uploadFilePortal').files[0];
        this.edittgtfile[0] = tgtfile.name;
        data.append('file', tgtfile);
      }

      var config = {
        onUploadProgress: function(progressEvent) {
          app.uploadPercentCompleted = progressEvent.loaded / progressEvent.total;
        },
        cancelToken: new axios.CancelToken(function executor(c) {
          cancelUploadFunc = c;
        })
      };

      axios.put('/storageManager/upload-file', data, config)
        .then(function (res) {
          var fileProposedSaveRoute = res.data;
          app.edittgtfile[2] = fileProposedSaveRoute;
          app.fileUploaded = true;
        })
        .catch(function (err) {
          console.log( err.message );throw err;
        });
    },
    cancelEditFileInfoIfUploaded: function () {
      if(app.uploadPercentCompleted == 0){return;}
      else{
        cancelUploadFunc();
        axios.post('/storageManager/cancel-upload-file', {proposedfile:this.edittgtfile[2]}
          // 先与服务器后端通信，确认上传，并获取文件储存路径。
          ).then(function (response) {window.alert("成功退出，请注意退出后文件无法保留，需重新上传文件。");console.log("取消成功")})
      }
    },
    saveEditFileInfo: function () {
      axios.post('/storageManager/confirm-upload-file', {level:this.edittgtitem.ind, stuff:this.edittgtitem, proposedfile:this.edittgtfile,warehs:this.CurWarehouse,baseInd:this.baseInd}
      // 先与服务器后端通信，确认上传，并获取文件储存路径。
      ).then(function (response) {
        app.edittgtitem.files.push(  JSON.parse(JSON.stringify(app.edittgtfile))  );
        app.uploadPercentCompleted=0;
        app.$bvModal.hide('editFileInfo');
      })
    },
    getfile: function (path) {
      window.open(encodeURI("/storageManager/get-file?dlname="+path[0]+'&path='+path[2]))
      // axios.post('/storageManager/get-file', {path:path}
      // // 先与服务器后端通信，确认上传，并获取文件储存路径。
      // ).then(function (response) {
      //   window.alert("还没开发呢！~")
      // })
    },
    generateThumbnail: function (path) {
      文件后缀 = path[0].substring(path[0].lastIndexOf("."));
      文件名 = path[0].substring(0,path[0].lastIndexOf("."));
      if(文件名==''){文件名 = path[0];}
      this.edittgtfile=['缩略图 of '+文件名+'.jpg','','','thumbnail'];
      if(['.png','.jpg','.jpeg','.svg','.tiff','.bmp','.gif'].indexOf(文件后缀)==-1){if(!window.confirm("不是图片文件！确认要继续吗？")){return}}
      axios({
          url: encodeURI("/storageManager/get-file?dlname="+path[0]+'&path='+path[2]), // url
          method: 'GET',
          responseType: 'blob', // important
      }).then((response) => {
        // 图片压缩：查询“前端 canvas 压缩图片”
        const imgBlob = new Blob([response.data]);
        var reader = new FileReader(), img = new Image();
        var file = null;
        var canvas = document.createElement('canvas');
        var context = canvas.getContext('2d');

        reader.onload = function(e) {
          img.onload = function () {
            var originWidth = this.width;
            var originHeight = this.height;
            var maxWidth = 800, maxHeight = 800;
            var targetWidth = originWidth, targetHeight = originHeight;
            if (originWidth > maxWidth || originHeight > maxHeight) {
                if (originWidth / originHeight > maxWidth / maxHeight) {
                    // 更宽，按照宽度限定尺寸
                    targetWidth = maxWidth;
                    targetHeight = Math.round(maxWidth * (originHeight / originWidth));
                } else {
                    targetHeight = maxHeight;
                    targetWidth = Math.round(maxHeight * (originWidth / originHeight));
                }
            }
                
            canvas.width = targetWidth;
            canvas.height = targetHeight;
            context.clearRect(0, 0, targetWidth, targetHeight);
            context.drawImage(img, 0, 0, targetWidth, targetHeight);
            canvas.toBlob(function (blob) {
              var imageUrl = URL.createObjectURL(blob);
              var data = new FormData();
              data.append('file', blob);
              axios.put('/storageManager/upload-file', data).then(function (res) {
                var fileProposedSaveRoute = res.data;
                app.edittgtfile[2] = fileProposedSaveRoute;
                axios.post('/storageManager/confirm-upload-file', {level:app.edittgtitem.ind, stuff:app.edittgtitem, proposedfile:app.edittgtfile,warehs:app.CurWarehouse,baseInd:app.baseInd}
                // 先与服务器后端通信，确认上传，并获取文件储存路径。
                ).then(function (response) {
                  app.edittgtitem.files.unshift(  JSON.parse(JSON.stringify(app.edittgtfile))  );
                  // app.$bvModal.hide('edittargetmodal');
                })
              })   
            }, 'image/jpeg');
          };

          img.src = e.target.result;
        };
        reader.readAsDataURL(imgBlob);
      });
    },
    previewImg: function (path) {
      this.$bvModal.show('previewImgModal');
      this.previewImgURL = encodeURI("/storageManager/get-file?dlname="+path[0]+'&path='+path[2])
    },
    changeDir: function (path) {
      var tgt = [...app.baseInd,...path]
      if(path=='../'){tgt = [...app.baseInd].slice(0,app.baseInd.length-1)}
      if(path=='root'){tgt = []}
      window.location = window.location.href.split('/').slice(0,4).join('/')+'/'+app.CurWarehouse+'/'+JSON.stringify(tgt)
    },
    copyItemLink: function (param) {
      if (param=='ind'){
      copyStringToClipboard(window.location.href.split('/').slice(0,4).join('/')+'/'+app.CurWarehouse+'/'+JSON.stringify([...app.baseInd,...app.edittgtitem.ind]));
      window.alert('已复制链接。请注意当您移动树结构时，这个链接指向的元素会变动，非永久链接。')
      }
      else if(param=='stable'){
      copyStringToClipboard(window.location.href.split('/').slice(0,4).join('/')+'/'+app.CurWarehouse+'/'+JSON.stringify(app.edittgtitem.名称+'|'+app.edittgtitem.setup));
      window.alert('已复制链接。请注意当您移动树结构时，这个链接指向的元素会变动，非永久链接。')
      }
    },
    addBatchSentences: function () {
      var text = this.batchSentences+ "。";
      this.batchSentences = '';
      text = text.replace("  ", " ").replace("  ", " ").replace("  ", " ").replace("　", "").replace('；', "。").replace('……', "…").replace("\n", "").replace("。。", "。").replace("！。", "！").replace("？。", "？").replace("…。", "…").replace("；。", "；");
      console.log(text)
      text = text.replace(/([0-9a-zA-Z\.\n\!\?…\~？。｡！—] *)。/g,(match,p1)=>{return p1})
      const arr = [...text.matchAll(new RegExp(".*?(｡|。|？|！|…|;|!|\\?|(No\\.(?=[^ a-z0-9]))|No\\. +(?=[^a-z])|\\. *$|\\!|\\! *$)",'g'))];
      for (const match of arr) {
        if (match[0].replace("。", "").length>1){ 
          this.edittgtitem.句向量描述.push(match[0].replace(".。", ".")); 
        }
      }
    },
    copyPasteMgmt: function () {
      this.$bvModal.show('copyPasteModal');
    },
    genCopyLink: function () {
      axios.post('/storageManager/gen-copy-link', {ind:this.edittgtitem.ind, wh:this.CurWarehouse}
      // 先与服务器后端通信，确认上传，并获取文件储存路径。
      ).then(function (response) {
        resptext = response.data;
        copyStringToClipboard(resptext)
        window.alert("已将链接复制到您的剪贴板~")
      })
    },
    submitPasteLink: function () {
      axios.post('/storageManager/submit-paste-link', {ind:this.edittgtitem.ind, wh:this.CurWarehouse, auth:this.copyPaste1}
      // 先与服务器后端通信，确认上传，并获取文件储存路径。
      ).then(function (response) {
        window.alert("已粘贴至本元素子树中~")
      })
    },
    editProgram: function () {
      const LoadScriptPromise = new Promise((resolve, reject) => {

if(!app.loadedJsLibs.hljs){
var newScript = document.createElement('link');
newScript.rel = 'stylesheet';
// newScript.href = "https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.3.1/build/styles/default.min.css";
newScript.href = "/static/仓储管理/style.css";
document.getElementsByTagName('head')[0].appendChild(newScript);
// 脚本加载完毕
newScript.onload = function () {

var newScript = document.createElement('script');
newScript.type = 'text/javascript';
newScript.src = '/static/仓储管理/highlight.min.js';
document.getElementsByTagName('head')[0].appendChild(newScript);

newScript.onload = function () {
var newScript = document.createElement('script');
newScript.type = 'text/javascript';
newScript.src = '/static/仓储管理/mathematica.min.js';
document.getElementsByTagName('head')[0].appendChild(newScript);
newScript = document.createElement('script');
newScript.type = 'text/javascript';
newScript.src = '/static/仓储管理/applescript.min.js';
document.getElementsByTagName('head')[0].appendChild(newScript);
newScript = document.createElement('script');
newScript.type = 'text/javascript';
newScript.src = '/static/仓储管理/vba.min.js';
document.getElementsByTagName('head')[0].appendChild(newScript);
// 脚本加载完毕
app.loadedJsLibs.hljs = true;
newScript.onload = function () {

// 引入vue plugin
Vue.directive('highlightjs', {
  deep: true,
  bind: function (el, binding) {
    // on first bind, highlight all targets
    let targets = el.querySelectorAll('code')
    targets.forEach((target) => {
      // if a value is directly assigned to the directive, use this
      // instead of the element content.
      if (binding.value) {
        target.textContent = binding.value
      }
      hljs.highlightBlock(target)
    })
  },
  componentUpdated: function (el, binding) {
    // after an update, re-fill the content and then highlight
    let targets = el.querySelectorAll('code')
    targets.forEach((target) => {
      if (binding.value) {
        target.textContent = binding.value
        hljs.highlightBlock(target)
      }
    })
  }
})

app.tmpProgType = app.edittgtitem.program[0];
app.tmpProgText = app.edittgtitem.program[1];
app.tmpProgText2 = app.edittgtitem.program[2];
app.tmpProgLang = app.edittgtitem.program[3];

  resolve('foo');
}

}

}
}else{resolve('foo');}

      });
      // 第一次编辑时需要load script(Highlight.js)。
      LoadScriptPromise.then(()=>{app.$bvModal.show('ProgramModal');})
    },
    editProgramShown: function () {
      hljs.highlightAll();
      app.tmpProgTextRealtime = app.tmpProgText;
      app.tmpProgText2Realtime = app.tmpProgText2;
      app.renderProg();
    },
    changeProgType: function (e) {
      console.log(app.edittgtitem.program[0],e);
      app.tmpProgText = app.tmpProgTextRealtime;
      app.tmpProgText2 = app.tmpProgText2Realtime;
      if(!window.confirm("您确定要换类型吗？之前的数据会丢失。")){
        app.tmpProgType = app.edittgtitem.program[0];
        return;
      }
      if(e=='clip'){app.tmpProgLang = 'Language-Mathematica'; app.tmpProgText = 'RandomInstance[\n GeometricScene[{a, b, c, d, e, f, g, h, i, j}, {Line[{a, b, c, d}], \n   Line[{d, e, f, g}], Line[{g, h, b, i}], Line[{i, c, e, j}], \n   Line[{j, f, h, a}], \n   PlanarAngle[{b, c, e}, "Counterclockwise"] == 100 \[Degree], \n   PlanarAngle[{f, h, b}, "Counterclockwise"] == 110 \[Degree], \n   PlanarAngle[{h, a, b}, "Counterclockwise"] == 35 \[Degree]}], \n RandomSeeding -> 1234]';}
      if(e=='template'){
        app.tmpProgText = '请输入文本模板~例如，{{公司名}}公司即将发行零息债券，面值{{faceValue}}，利率{{rate}}，期限{{year}}年\n  ∴则{{公司名}}零息债价格为{{price}}。';
        app.tmpProgText2 = 'var app = new Vue({el: "#app",\ndata: { /*↓初始数据*/\n\n公司名: "神御中华",\nfaceValue: 1000,\nrate: 0.03,\nyear: 3,\nprice: 0\n\n},mounted() {/*↓加载后执行*/\n\nthis.price = this.faceValue/((1+this.rate)**this.year)\n\n},methods: {}\n});';
      }
      if(e=='bvapp'){
        app.tmpProgText = '<h2>即时的HTML编辑器是伟大的！{{randnum}}</h2>\n<button @click="update()">生成新的随机数！</button>\n<div id="test" v-html="output"></div>';
        app.tmpProgText2 = 'var app = new Vue({\n    el: "#app",\n    data: {\n      msg: "hello world",\n      randnum: 0,\n      output: ""\n    },\n    mounted() {\n      var output = "";\n      for (var i = 0; i < 3; i++) { output += "JavaScript的作品呢！<br>"; }\n      document.getElementById("test").innerHTML = output;\n    },\n    methods: {\n      update: function () {\n        this.randnum = Math.random();\n        this.output  = "";\n        for (var i = 0; i < Math.round(this.randnum*10); i++) { this.output  += "JavaScript的作品呢！<br>"; }\n      },\n    }\n  });';
      }
      if(e=='htmljs'){
        app.tmpProgText = '<html lang="zh">\n<head>\n<meta charset="UTF-8">\n<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap@4.5.3/dist/css/bootstrap.min.css"\n  integrity="sha256-93wNFzm2GO3EoByj9rKZCwGjAJAwr0nujPaOgwUt8ZQ=" crossorigin="anonymous">\n<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap-vue@2.21.2/dist/bootstrap-vue.min.css"\n  integrity="sha256-cAyk5KZc7P6j7j/uL7MOoN4PRsZYp+BN9yo03Y6Qk38=" crossorigin="anonymous">\n<style>*{font-family:"Sarasa Term SC";}body{white-space:pre-wrap}</style>\n</head>\n<body><div id="app"><!-- vue instance -->\n<h2>即时的HTML编辑器是伟大的！{{randnum}}</h2>\n<button @click="update()">生成新的随机数！</button>\n<div id="test" v-html="output"></div>\n<!-- vue instance end --></div></body>\n<sc'+'ript src="https://cdn.jsdelivr.net/npm/vue@2.6.14/dist/vue.min.js"\n  integrity="sha256-kXTEJcRFN330VirZFl6gj9+UM6gIKW195fYZeR3xDhc=" crossorigin="anonymous"></sc'+'ript>\n<sc'+'ript src="https://cdn.jsdelivr.net/npm/bootstrap-vue@2.21.2/dist/bootstrap-vue.min.js"\n  integrity="sha256-duDNIhfVrUK7HB/57nPLxN/j92aM3rhTUFzVI/H5ex8=" crossorigin="anonymous"></sc'+'ript>\n<sc'+'ript>\nvar app = new Vue({\n  el: "#app",\n  data: {\n    msg: "hello world",\n    randnum: 0,\n    output: ""\n  },\n  mounted() {\n    var output = "";\n    for (var i = 0; i < 3; i++) { output += "JavaScript的作品呢！<br>"; }\n    document.getElementById("test").innerHTML = output;\n  },\n  methods: {\n    update: function () {\n      this.randnum = Math.random();\n      this.output  = "";\n      for (var i = 0; i < Math.round(this.randnum*10); i++) { this.output  += "JavaScript的作品呢！<br>"; }\n    },\n  }\n});\n</sc'+'ript>\n</html>';
      }
      app.tmpProgTextRealtime = app.tmpProgText;
      app.tmpProgText2Realtime = app.tmpProgText2;
      app.edittgtitem.program[0] = e;
      app.$forceUpdate();setTimeout(()=>{app.renderProg()}, 300);
      hljs.highlightAll();
    },
    renderProg: function () {
      本次input的时间 = new Date();
      setTimeout(function(){ 
      当前时间 = new Date();
      if((当前时间 - 上次刷新的时间 > 499)&&( 当前时间 - 本次input的时间 > 499)){ // 这是最后一次输入。
        var html, encodedHtml
        if(app.tmpProgType=='template'){
          html = 前缀str + app.tmpProgTextRealtime.replace(/\\{/g,'{{ "\\{" }}').replace(/\\}/g,'{{ "\\}" }}') + 中缀str + app.tmpProgText2Realtime.replace(/\\{/g,'{{ "\\{" }}').replace(/\\}/g,'{{ "\\}" }}') + 后缀str;
          encodedHtml = encodeURIComponent(html);
        }
        if(app.tmpProgType=='bvapp'){
          html = 前缀str + app.tmpProgTextRealtime + 中缀str + app.tmpProgText2Realtime + 后缀str;
          encodedHtml = encodeURIComponent(html);
        }
        if(app.tmpProgType=='htmljs'){encodedHtml = encodeURIComponent(app.tmpProgTextRealtime)}
        if(app.tmpProgType!='clip'){
          var viewEl = document.getElementById('ProgIframeView');
          viewEl.style.height = '700px';
          viewEl.src = 'data:text/html,' + encodedHtml;
          上次刷新的时间 = new Date();
        }
      }
      }, 500);
    },
    saveProgram: function (e) {
      if(app.tmpProgType=='clip'){
        this.tmpProgText=getProgText('#P1_ProgramClipCode')
        this.tmpProgText2=''
      }
      if(app.tmpProgType=='template'){
        this.tmpProgText=getProgText('#P2_html')
        this.tmpProgText2=getProgText('#P2_js')
      }
      if(app.tmpProgType=='bvapp'){
        this.tmpProgText=getProgText('#P3_html')
        this.tmpProgText2=getProgText('#P3_js')
      }
      if(app.tmpProgType=='htmljs'){
        this.tmpProgText=getProgText('#P4_ProgramCode')
        this.tmpProgText2=''
      }
      this.edittgtitem.program = [this.tmpProgType, this.tmpProgText, this.tmpProgText2, this.tmpProgLang];
      setTimeout(() => {app.$bvModal.hide('ProgramModal');app.saveedittgt();}, 200);
    },
    editRTF: function () {
      const LoadScriptPromise = new Promise((resolve, reject) => {

if(!app.loadedJsLibs.kindeditor){
var newScript = document.createElement('link');
newScript.rel = 'stylesheet';
newScript.href = "/editor/themes/default/default.css";
document.getElementsByTagName('head')[0].appendChild(newScript);
// 脚本加载完毕
newScript.onload = function () {

var newScript = document.createElement('script');
newScript.type = 'text/javascript';
newScript.src = '/editor/kindeditor-all-min.js';
document.getElementsByTagName('head')[0].appendChild(newScript);
// 脚本加载完毕
app.loadedJsLibs.kindeditor = true;
newScript.onload = function () {

var newScript = document.createElement('script');
newScript.type = 'text/javascript';
newScript.src = '/editor/lang/zh-CN.js';
document.getElementsByTagName('head')[0].appendChild(newScript);
// 脚本加载完毕
app.loadedJsLibs.kindeditor = true;
newScript.onload = function () {
resolve('foo');
}

}

}
}else{resolve('foo');}

      });
      // 第一次编辑时需要load script(KindEditor)。
      LoadScriptPromise.then(()=>{app.$bvModal.show('RTFModal');})
    },
    editRTFShown: function () {
      // KindEditor.ready(function(K) {
      //   window.editor = K.create('#editor_id');
      // });
      KindEditor.options.filterMode = false;
      editor = KindEditor.create('#editor_id');
      document.querySelector("#RTFModal___BV_modal_body_ > div").style.width = '100%';
      document.querySelector("#RTFModal___BV_modal_body_ > div > div.ke-edit").style.height = '800px';
      document.querySelector("#RTFModal___BV_modal_body_ > div > div.ke-edit > iframe").style.height = '800px';
      document.querySelector("#RTFModal___BV_modal_body_ > div > div.ke-edit > textarea").style.height = '798px';
      this.editorText = ''
      if(this.edittgtitem.rtf){this.editorText = this.edittgtitem.rtf}
      editor.html(this.editorText)
    },
    saveRTF: function (e) {
      this.ConfirmText = "请问是否要把纯文本的rtf放到简介描述中呢~ 是选确认or直接点击背景关闭modal；否选取消。（会覆盖。但不用担心误操作，您原先的描述文本会被复制到剪贴板。）";
      this.$bvModal.show('ConfirmModal');
      var ConfirmPromise = new Promise((resolve, reject) => {app.ConfirmPromiseResolve = resolve});
      ConfirmPromise.then(()=>{
        if(app.ConfirmResult){
          copyStringToClipboard(app.edittgtitem.描述);
          app.edittgtitem.描述 = editor.text().replace(/\t+/g,'').replace(/\n+/g,'\n');
        }
        app.editorText=editor.html();
        app.edittgtitem.rtf = app.editorText;
        setTimeout(() => {app.$bvModal.hide('RTFModal');app.saveedittgt();}, 200);
      })
    },
    simpleExample: function () {
      if (this.query_txt == "") {
        this.query_txt = "[<洗发露>]+[<飘柔>]*0.5+[<清爽>]*0.3"
      } else {
        if (window.confirm("您的搜索栏中有文字。继续操作将清空您之前输入的内容。真的要继续吗？")) {
          this.query_txt = "[<洗发露>]+[<飘柔>]*0.5+[<清爽>]*0.3"
        }
      }
    },
    rebuildNGT: function () {
      this.isQueryingBackend = true;
      axios.post('/storageManager/rebuildNGT', {warehs:this.CurWarehouse}).then(function (response) {
        app.$bvToast.toast('已重构搜索数据结构，请重试搜索~', {title: `已rebuild NGT~`, appendToast: true, variant: "info", solid: true, toaster: 'b-toaster-bottom-right'})
        app.isQueryingBackend = false;
      }).catch(function (error) {console.log(error);})
    },
    rebuildNGTReloadVec: function () {
      this.isQueryingBackend = true;
      axios.post('/storageManager/rebuildNGTReloadVec', {warehs:this.CurWarehouse}).then(function (response) {
        app.$bvToast.toast('已重构搜索数据结构ReloadVec，请重试搜索~', {title: `已rebuild NGT~`, appendToast: true, variant: "info", solid: true, toaster: 'b-toaster-bottom-right'})
        app.isQueryingBackend = false;
      }).catch(function (error) {console.log(error);})
    },
    searchSubtree: function (tgt) {
      if (isLoading == false) {
        if (this.query_txt.length > 300) {window.alert("QUERY TOO LONG!!"); return;}
        if (this.query_txt.length == 0) {window.alert("您尚未输入搜索字符串！"); return;}
        isLoading = true;
        this.isQueryingBackend = true;
        setTimeout(() => { isLoading = false; }, 2000);
        axios.post('/storageManager/query-subtree', {
          query: this.query_txt,
          tgt: tgt,
          resnumber: this.resnumber,
          warehs:this.CurWarehouse,baseInd:this.baseInd
        }).then(function (response) {
          if (app.page_history<2) {window.history.pushState({page: app.page_history}, "", "");}  // 浏览器返回
          app.page_history=Math.min(app.page_history+1,2);
          
          resptext = response.data;
          if (resptext == 'nope') {window.alert("您尚未登录哟~"); app.isQueryingBackend = false; return;}
          tmpobj = eval(resptext);founderr=false;
          tmpobj.forEach(element => {
            if(element=="notfound"&&!founderr){founderr=true;window.alert("后端搜索出错！已经帮助您重新搭建搜索NGT结构了~ 请您刷新页面，稍等再试！");app.isQueryingBackend = false;app.rebuildNGT();return;}
          });
          app.search_result = tmpobj;
          app.show_search_result_mode = true;
          app.isQueryingBackend = false;
          setTimeout(() => {
            window.scrollTo(0, 0);
          }, 270);
        }).catch(function (error) {
          console.log(error);app.isQueryingBackend = false;
        })
      } else {
        console.log("TOO MANY CLICKS!!")  // 防止多重点击！！
        window.alert("搜索过于频繁!!!仔细等结果2秒啊。")  // 防止多重点击！！
      }
    },
    GetTodayStr: function () {
      var x = new Date();
      currentTimeZoneOffsetInHours = - x.getTimezoneOffset() / 60;
      var day = new Date();
      day.setHours(day.getHours() + currentTimeZoneOffsetInHours); 
      var str=day.toISOString().slice(0,10);
      var arr=str.split('-').map(Math.abs);
      return arr[0]+'年'+arr[1]+'月'+arr[2]+'日'+' '+this.weekDayMap[day.getDay()]
    },
    GetDateDescription: function (item) {
      try {
      var x = new Date();
      currentTimeZoneOffsetInHours = - x.getTimezoneOffset() / 60;
      var day = new Date();
      day.setHours(day.getHours() + currentTimeZoneOffsetInHours); 
      var str=day.toISOString();
      var pred = '';
      var splitted = item.截止日期.split('-');
      if(splitted && splitted[0] != str.slice(0,4)){pred = item.截止日期.split('-')[0] + '年' }
      if(item.截止日期 == str.slice(0,10)){pred = '今天 ' + pred }
      if(item.截止日期 < str.slice(0,10)){pred = '已过期 ' + pred }
      return pred+item.截止日期.split('-').slice(1).map(Math.abs).join('月')+'日'+' '+this.weekDayMap[new Date(item.截止日期).getDay()]
      } catch (error) {
      return '未知日期'
      }
    },
    timeMgmtClick: function (level) {
      console.log("edittarget",level)
      if(level.length==0){this.edittgtitem=this.fullitems;return}
      target = app.fullitems;
      for (const i of level.slice(0,-1)) {
        target = target[i].内容
      }
      this.edittgtitem=target[level[level.length-1]];
      var et = this.edittgtitem;
      if(et.重复数 <= 0) {
        // 没有重复，直接删除。
        this.deleteitems(level)
      }
      else {
        // 有重复，只需更改时间，直接save-items。
        var x = new Date();
        currentTimeZoneOffsetInHours = - x.getTimezoneOffset() / 60;  
        var old = new Date(et.截止日期+' '+et.截止时间);
        if(et.重复间隔=="日"){old.setDate(old.getDate()+parseInt(et.重复数));}
        if(et.重复间隔=="周"){old.setDate(old.getDate()+parseInt(et.重复数)*7);}
        if(et.重复间隔=="月"){old.setMonth(old.getMonth()+parseInt(et.重复数));}
        if(et.重复间隔=="季度"){old.setMonth(old.getMonth()+parseInt(et.重复数)*3);}
        if(et.重复间隔=="年"){old.setFullYear(old.getFullYear()+parseInt(et.重复数));}
        if(et.重复间隔=="小时"){old.setHours(old.getHours()+parseInt(et.重复数));}
        old.setHours(old.getHours() + currentTimeZoneOffsetInHours); var str = old.toISOString();
        et.截止日期 = str.slice(0,10); et.截止时间 = str.slice(11,16); 
        axios.post('/storageManager/save-items', {level:et.ind,stuff:et,warehs:this.CurWarehouse,baseInd:this.baseInd}).then(function (response) {
          resptext = response.data;
          console.log(resptext)
          var day = new Date(); day.setHours(day.getHours() + currentTimeZoneOffsetInHours); var str = day.toISOString();
          app.edittgtitem.modify = str.slice(0,10)+' '+str.slice(11,19);
          app.$bvToast.toast('已保存位于程序数据结构['+app.edittgtitem.ind+']处的信息~', {title: `已保存~`, autoHideDelay: 2500, appendToast: true, variant: "info", solid: true, toaster: 'b-toaster-bottom-right'})
          if(app.edittgtitem.ind.length==1){if(app.isTimeManagement){app.TimeSortAsc();}} // 若todolist则自动排序
        }).catch(function (error) {console.log(error);})
      }
    },
    dateClass(ymd, date) {
      const day = date.getDate()
      if(daysMap.has(ymd)){
        if(daysMap.get(ymd)==1){return 'table-success'}
        if(daysMap.get(ymd)==2){return 'table-primary'}
        if(daysMap.get(ymd)==3){return 'table-warning'}
        if(daysMap.get(ymd)>3){return 'table-danger'}
        return ''
      }
    },
    TimeSortAsc: function () {
app.finishedTimeAsc=false;
if(app.fullitems.length<=1){app.finishedTimeAsc=true;} app.$forceUpdate();
//对todolist，自动调用move-item给底层排序。
daysMap = new Map()// 用于决定dateClass给日历上色，统计事件数量
var 原现ind记录 = []

for (let i = 0; i < app.fullitems.length; i++) {
  var ddl = app.fullitems[i].截止日期;
  if(daysMap.has(ddl)){daysMap.set(ddl, daysMap.get(ddl)+1)}
  else{daysMap.set(ddl, 1)}
  原现ind记录.push({后端初始ind: i, 截止日期:ddl, 截止时间:app.fullitems[i].截止时间})
}
var 原ind复制 = [...原现ind记录]
原现ind记录.sort(function (e1, e2) {
  try {
    return new Date(e1.截止日期+' '+e1.截止时间) - new Date(e2.截止日期+' '+e2.截止时间);
  } catch (error) {
    return 0
  }
});
for (let i = 0; i < 原现ind记录.length; i++) {
  原现ind记录[i].后端目标ind = i
}

console.log("更新顺序")

var action = (i) => {
  const myPromise = new Promise((resolve, reject) => {

    let 目前移动到的ind = i.i
    i.i = i.i+1
    for (let i = 目前移动到的ind; i < 原ind复制.length; i++) {
      var element = 原ind复制[i];
      if(element.后端目标ind == 目前移动到的ind){srci=i;tgti=element.后端目标ind;break}
    }
    // 若src在tgt后1位，则移动tgt元素，以节省可能的换序操作
    if(srci==tgti+1){srci=tgti;tgti=原ind复制[tgti].后端目标ind;}
    // 因为是从tgt为0开始逐渐增加，因此 src 必>= tgt
    srcitem = 原ind复制.splice(srci,1) // 删除srci
    原ind复制.splice(tgti,0,srcitem[0]) // tgti处插入srcitem
    // 现在 原ind复制 里面表述的是后端的存放信息，下一步是要把 原ind复制 中需要挪动到下一个tgt的元素找到。
    // console.log(srci, tgti, srcitem)
    if(srci!=tgti){
      axios.post('/storageManager/move-items', {src:[srci],tgt:[tgti],warehs:app.CurWarehouse,baseInd:app.baseInd}).then(function (response) {
        resptext = response.data; resvals = eval(resptext);
        app.fullitems= resvals[0];
        if(i.i==原现ind记录.length-1){app.finishedTimeAsc=true}
        app.$forceUpdate();
        resolve(i);
      }).catch(function (error) {
        console.log(error);window.alert("获取后端信息出错，为了保证数据结构不受损害，请刷新页面！！")
      })
    }
    else{
    if(i.i==原现ind记录.length-1){app.finishedTimeAsc=true};
    resolve(i);}

  });
  return myPromise
}

const promiseFor = (i, n) => {
  var whilst = (i) => {
    return i.i<n ?
      action(i).then(whilst) :
      Promise.resolve(i);
  }
  return whilst(i);
};
i = {i:0}
promiseFor(i, 原现ind记录.length).then(console.log("finish."))

    }
  }
})

document.addEventListener("keydown", function(event) {
  if (event.keyCode === 13  && ((!app.entershoulddefault) || event.shiftKey) ) { 
    event.preventDefault();
    app.searchSubtree([]);
  }
  if (event.keyCode === 112) {
    event.preventDefault();
    app.executeCmdCommand();
  }
  if (event.keyCode === 113) {
    app.consoleCmd = "";
    document.getElementById("cmdcommand").focus();
  }
  if (event.keyCode === 114) {
    document.getElementById("mainsearchbox").focus();
  }
});
document.onpaste = (evt) => {
  const dT = evt.clipboardData || window.clipboardData;
  const file = dT.files[ 0 ];
  if(file){app.uploadFile(file)}
};

window.addEventListener("beforeunload", function(event) { if(reload_upload_focus_flip) {reload_upload_focus_flip=false;} });
window.onpopstate = function(event) {
  console.log(event);
  if (app.page_history == 2) {
    app.page_history = 1;
    app.show_search_result_mode=false;
    app.$bvToast.toast(`本网站回退功能并不完善，如果多次搜索，会导致之前的搜索结果丢失，无法通过历史浏览记录复原。 因此请注意保存结果！          您上一次搜索的内容可以通过点击前进按钮恢复。`, {
      title: '⚠️请注意保存结果！',
      autoHideDelay: 10000,
      appendToast: true,
      variant: "danger",
      solid: true,
      toaster: 'b-toaster-bottom-right'
    })
  } else {
    app.page_history = 2;app.show_search_result_mode=true;
  }
}