<template>
	<div id="app">
		<Home v-if="global.isAppPasswordValidation" />

		<el-dialog title="请设置程序锁" :visible.sync="showSetAppPassDialog" width="50%" :show-close="false"
			:close-on-click-modal="false" :close-on-press-escape="false" :center="true" top="30vh">
			<el-input placeholder="请输入密码" v-model="appPassword" show-password :minlength="8" :show-word-limit="true"
				@keyup.enter.native="setPassword">
			</el-input>
			<p style="text-align:center; margin-top: 10px; color: red">{{ checkPasswordInfo }}</p>
			<span slot="footer" class="dialog-footer">
				<el-button type="text" @click="setPassword">确 定</el-button>
			</span>
		</el-dialog>

		<el-dialog title="请输入程序锁" :visible.sync="showValidAppPassDialog" width="50%" :show-close="false"
			:close-on-click-modal="false" :close-on-press-escape="false" :center="true" top="30vh">
			<el-input placeholder="请输入密码" v-model="appPassword" show-password :minlength="8" :show-word-limit="true"
				@keyup.enter.native="validPassword">
			</el-input>
			<span slot="footer" class="dialog-footer">
				<el-button type="text" @click="validPassword">确 定</el-button>
			</span>
		</el-dialog>
	</div>
</template>

<script>
import { invoke } from '@tauri-apps/api/tauri';
import { mapState } from 'vuex';
import Home from './views/Home.vue';
import { registerAll, unregisterAll } from '@tauri-apps/api/globalShortcut';
// import { fs } from '@tauri-apps/api'

export default {
	name: 'App',
	components: {
		Home
	},
	data() {
		return {
			showSetAppPassDialog: false,
			showValidAppPassDialog: false,
			appPassword: '',
		}
	},

	methods: {
		setPassword() {
			if (this.checkPasswordInfo.length == 0 && this.appPassword.length != 0) {
				invoke('set_app_password', { 'pass': this.appPassword }).then(resp => {
					if (resp) {
						this.$message({
							message: '设置成功',
							type: 'success'
						});
						this.showSetAppPassDialog = false;
						this.$store.dispatch('global/setIsSetAppPassword', true);
						this.$store.dispatch('global/passValidation')
						this.appPassword = "";
					}
					else this.$message.error({
						message: '设置失败',
					});
				})
			} else {
				this.$message.error({
					message: '密码不符号要求',
					type: 'warning'
				});
			}
		},
		validPassword() {
			if (this.appPassword.length < 8) {
				this.$message.error({
					message: '密码不符号要求',
					type: 'warning'
				});
			} else {
				invoke('valid_app_password', { 'pass': this.appPassword }).then(resp => {
					if (resp) {
						// this.$message({
						// 	message: '密码正确',
						// 	type: 'success'
						// });
						if (resp) {
							this.showValidAppPassDialog = false;
							this.appPassword = "";
							this.$store.dispatch('global/passValidation')
						}
					}
					else this.$message.error({
						message: '密码错误',
					});
				})
			}
		}
	},
	computed: {
		...mapState(['global', 'user']),
		checkPasswordInfo() {
			if (this.appPassword.length == 0) return "";
			else if (this.appPassword.length < 8) return "密码长度需大于8位";
			return "";
		},
	},
	mounted() {
		// 设置无法右键
		window.oncontextmenu = () => {
			return false;
		};

		const loading = this.$loading({
			lock: true,
			text: 'Loading',
			spinner: 'el-icon-loading',
			background: 'rgba(0, 0, 0, 0.7)'
		});

		let _this = this;
		let h = setInterval(() => {
			if (_this.global.initOver) {
				clearInterval(h);
				loading.close();
				_this.showSetAppPassDialog = !_this.global.isSetAppPassword;
				_this.showValidAppPassDialog = _this.global.isSetAppPassword;
				// _this.showSetAppPassDialog = true;
			}
		}, 100);

		this.$store.dispatch('global/initialize');

		if (this.global.debug == false) {
			unregisterAll();
			// 设置特殊按键监听事件 F12, Ctrl+R, F5
			registerAll(['CommandOrControl+R', 'F5', 'F12', 'F3', 'F7'], (sc) => {
				console.log(`Shortcut ${sc} triggered`);
			})
		} else {
			this.appPassword = '22222222'
		}
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

/* Original Style */

hr {
	/* height: 10px; */
	border-bottom: #3a3a3a solid 1px;
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

.el-table__empty-block {
	width: 100%;
	height: 100%;
}


/* Dialog style */
.el-dialog {
	color: #cfcfcf;
	background-color: #0f0f0f;
	border: #646464 solid 1px;
}

.el-dialog__title {
	line-height: 24px;
	font-size: 18px;
	color: #cfcfcf;
}

.el-input__inner {
	background-color: #3f3f3f;
	color: #cfcfcf;
}

/* Selector Style */
.el-select-dropdown__item.hover,
.el-select-dropdown__item:hover {
	background-color: #3f3f3f;
}

.el-select-dropdown {
	/* border: 1px solid #E4E7ED; */
	background-color: #4f4f4f;
}

.el-select-dropdown__item {
	color: #cfcfcf;
}

/* Form Style */
.el-form-item__label {
	color: #cfcfcf;
}

/* AutoComplete Style */
.el-autocomplete-suggestion {
	background-color: #2f2f2f;
	color: #cfcfcf;
}

.el-autocomplete-suggestion li.highlighted,
.el-autocomplete-suggestion li:hover {
	background-color: #3f3f3f;
	color: #cfcfcf;
}

.el-autocomplete-suggestion li {
	color: #cfcfcf;
}

/* popconfirm Style */
.el-popover {
	background: #0f0f0f;
	color: #cfcfcf;
}
</style>
