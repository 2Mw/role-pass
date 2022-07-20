import Vue from 'vue'
import Vuex from 'vuex'
import { invoke } from '@tauri-apps/api/tauri'

Vue.use(Vuex)


const globalOptions = {
	namespaced: true,
	state: {
		// 是否初始化完毕
		initOver: false,
		// 是否设置程序锁
		isSetAppPassword: false,
		// 是否通过验证程序锁，如果未设置程序锁则为真
		isAppPasswordValidation: false,
		// 是否通过用户密码验证
		isUserPasswordValidation: false,
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
				state.isAppPasswordValidation = state.isSetAppPassword == false
			})

			invoke('query_users').then(rsp=> {
				console.log(rsp);
				this.state.user.users = rsp;
			})
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
		roles: [],
		passwords: [],
		// TODO Set current user name - role
		key: '',
	},
	actions: {
		storeKey(ctx, v) {
			ctx.commit('STOREKEY', v);
		}
	},
	mutations: {
		STOREKEY(state, v) {
			state.key = v;
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
