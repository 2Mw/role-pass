import Vue from 'vue'
import Vuex from 'vuex'
import { invoke } from '@tauri-apps/api/tauri'
import util from '../utils/Utils'

Vue.use(Vuex)


const globalOptions = {
	namespaced: true,
	state: {
		// debug or not
		debug: false,
		// 是否初始化完毕
		initOver: false,
		// 是否设置程序锁
		isSetAppPassword: false,
		// 是否通过验证程序锁，如果未设置程序锁则为真
		isAppPasswordValidation: false,
		// 是否通过用户密码验证
		isUserPasswordValidation: false,
		// 程序信息
		version: 'v0.1-alpha',
		releaseTime: '20220801',
	},
	actions: {
		initialize(ctx) {
			ctx.commit('INITIALIZE')
		},

		setIsSetAppPassword(ctx, value) {
			ctx.commit('SETISSETAPPPASSWORD', value)
		},

		passValidation(ctx) {
			ctx.commit('PASSVALIDATION')
		},

		passUserValidation(ctx) {
			ctx.commit('PASSUSERVALIDATION')
		},

		exitUserLogin(ctx) {
			ctx.commit('EXITUSERLOGIN')
		}
	},
	mutations: {
		INITIALIZE(state) {
			invoke('if_app_password_set').then(resp => {
				// console.log("Set password?", resp);
				state.isSetAppPassword = resp;
				// state.isAppPasswordValidation = state.isSetAppPassword == false
			})
			this.dispatch('user/queryUsers');
			state.initOver = true;
		},

		SETISSETAPPPASSWORD(state, value) {
			state.isSetAppPassword = value;
		},

		PASSVALIDATION(state) {
			state.isAppPasswordValidation = true;
		},

		PASSUSERVALIDATION(state) {
			state.isUserPasswordValidation = true;
		},

		EXITUSERLOGIN(state) {
			state.isUserPasswordValidation = false;
		}
	},
	getters: {

	}

}

const userOptions = {
	namespaced: true,
	state: {
		users: [],
		// 当前用户信息
		current_user: {},
		// 当前用户选择的角色
		current_role: '',
		key: '',
		// 当前用户对应的所有角色
		roles: [],
		// 当前用户对应的所有账号信息
		passwords: [],
		// 搜索框信息
		searchKey: ""

	},
	actions: {
		storeKey(ctx, v) {
			ctx.commit('STOREKEY', v);
		},

		storeUser(ctx, id) {
			ctx.commit('STOREUSER', id);
		},

		queryUsers(ctx) {
			ctx.commit('QUERYUSERS')
		},

		getUserRoles(ctx) {
			ctx.commit('GETUSERROLES')
		},

		getUserAccounts(ctx) {
			ctx.commit('GETUSERACCOUNTS');
		},

		setCurrentRole(ctx, role) {
			ctx.commit('SETCURRENTROLE', role);
		},

		setSearchKey(ctx, v) {
			ctx.commit('SETSEARCHKEY', v);
		}
	},
	mutations: {
		STOREKEY(state, v) {
			state.key = util.hash(v);
			// console.log(util.hash(v));
		},

		STOREUSER(state, v) {
			state.users.forEach(e => {
				if (e.id == v) {
					state.current_user = e;
				}
			});

			this.dispatch('user/getUserRoles');
			this.dispatch('user/getUserAccounts');

		},

		QUERYUSERS(state) {
			invoke('query_users').then(rsp => {
				// console.log(rsp);
				state.users = rsp;
			}).catch(e => {
				console.error(e);
			})
		},

		GETUSERROLES(state) {
			invoke('get_roles_by_id', { 'id': state.current_user.id })
				.then(resp => {
					state.roles = resp
					// console.log('role', resp);
					if (resp.length > 0) {
						let stillExistsRole = false;
						if (state.current_role.length != 0) {
							for (let i = 0; i < state.roles.length; i++) {
								const e = state.roles[i];
								if(e.role == state.current_role) {
									stillExistsRole = true;
									break;
								}
							}	
						}
						if(state.current_role == 0 || !stillExistsRole) {
							// 未设置当前角色，或者当前角色已被全部删除
							if (state.roles.length > 0) state.current_role = state.roles[0].role;
							else state.current_role = '';
						}
					}
				})
				.catch(e => {
					console.error(e);
				})
		},

		GETUSERACCOUNTS(state) {
			invoke('get_accounts_by_id', { 'id': state.current_user.id, 'key': state.key })
				.then(resp => {
					state.passwords = resp;
					// console.log(resp);
				}).catch(e => {
					console.error(e);
				})
		},

		SETCURRENTROLE(state, v) {
			let roles = state.roles.map(i => { return i.role });
			if (roles.indexOf(v) >= 0) {
				state.current_role = v;
			}
		},

		SETSEARCHKEY(state, v) {
			state.searchKey = v;
		}


	},

}

export default new Vuex.Store({
	state: {
	},
	mutations: {
	},
	actions: {
	},
	modules: {
		global: globalOptions,
		user: userOptions,
	}
})
