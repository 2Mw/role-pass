<template>
  <div id="app">
    <Home />
  </div>
</template>

<script>
import Home from './views/Home.vue'
// import { fs } from '@tauri-apps/api'

export default {
  name: 'App',
  components: {
    Home
  },
  data() {
    return {}
  },
  computed: {

  },
  mounted() {
    // 设置无法右键
    window.oncontextmenu = () => {
      return false;
    };
    
    
  }
}
</script>


<style>
* {
  margin: 0;
  padding: 0;
  font-family: "Helvetica Neue", Helvetica, "PingFang SC", "Hiragino Sans GB", "Microsoft YaHei", "微软雅黑", Arial, sans-serif;
}

html {
  height: 100vh;
  width: 100vw;
  background-color: #0f0f0f;
  color: #cfcfcf;
  user-select: none;
}

#app {
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
}

/* DropDown style modify */

.el-dropdown-link {
  color: #cfcfcf;
  cursor: pointer;
}

.el-dropdown-menu {
  background-color: #2f2f2f;
}

.el-dropdown-menu__item {
  color: #cfcfcf;
}

.el-dropdown-menu__item:focus,
.el-dropdown-menu__item:not(.is-disabled):hover {
  background-color: #3f3f3f;
  color: #66b1ff;
}

.el-popper[x-placement^=bottom] .popper__arrow::after {
  top: 1px;
  margin-left: -6px;
  border-top-width: 0;
  border-bottom-color: #3f3f3f;
}

/* Table Style */
.el-table {
  color: #a1a1a1;
}

.el-table thead {
  color: #cfcfcf;
  font-weight: 500;
}

.el-form-item__label {
  color: #919191;
}

.el-table--enable-row-hover .el-table__body tr:hover>td.el-table__cell {
  background-color: #1f1f1f;
}

.el-table tr,
.el-table th.el-table__cell {
  background-color: #0f0f0f;
}

.el-table--striped .el-table__body tr.el-table__row--striped td.el-table__cell {
  background-color: #161616;
}

.el-table--striped .el-table__body tr.el-table__row--striped:hover td.el-table__cell {
  background-color: #1f1f1f;
}

.el-table,
.el-table__expanded-cell {
  background-color: #0f0f0f;
}

.el-table__expanded-cell:hover {
  background-color: #0f0f0f !important;
}
</style>
