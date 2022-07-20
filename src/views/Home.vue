<template>
	<div class="home">
		<Header />
		<div v-if="global.isUserPasswordValidation">
			<br />
			<h1>User - Role</h1>
			<br />
			<hr>
			<PassTable />
		</div>

		<!-- 存在用户且未登录 -->
		<el-dialog title="请选择用户" :visible.sync="user.users.length > 0 && !global.isUserPasswordValidation" width="35%"
			:show-close="false" :close-on-click-modal="false" :close-on-press-escape="false" :center="true" top="20vh">
			<el-form label-position="left" label-width="80px">
				<el-form-item label="用户">
					<el-select v-model="selectedUser" placeholder="请选择" style="width: 100%;">
						<el-option v-for="item in user.users" :key="item.id" :label="item.name" :value="item.id">
						</el-option>
					</el-select>
				</el-form-item>
				<el-form-item label="密码">
					<el-input placeholder="请输入密码" v-model="selectedUserPassword" show-password :minlength="8"
						:show-word-limit="true" @keyup.enter.native="login">
					</el-input>
				</el-form-item>
			</el-form>
			<span slot="footer" class="dialog-footer">
				<el-button type="text" @click.native="login">确 定</el-button>
			</span>
		</el-dialog>
	</div>
</template>

<script>
import { invoke } from '@tauri-apps/api';
import { mapState } from 'vuex';
import Header from '../components/Header.vue';
import PassTable from '../components/PassTable.vue';
// @ is an alias to /src
// import HelloWorld from '@/components/HelloWorld.vue'

export default {
	name: 'Home',
	components: {
		Header,
		PassTable,
	},
	data() {
		return {
			selectedUser: '',
			selectedUserPassword: '11111111',
		}
	},
	methods: {
		login() {
			console.log(this.selectedUser, this.selectedUserPassword);
			if (this.selectedUser) {
				if (this.selectedUserPassword.length >= 8) {
					invoke('login', { 'id': this.selectedUser, 'pass': this.selectedUserPassword })
						.then(rsp => {
							console.log(rsp);
							if (rsp) {
								// console.log("登录成功");
								this.$store.dispatch('global/passUserValidation');
								this.$store.dispatch('user/storeKey', this.selectedUserPassword);
								this.selectedUser='';
								this.selectedUserPassword='';
							}
						}).catch(e => {
							this.$message.error({
								message: e
							})
						})
				} else {
					this.$message.error({
						message: '密码需大于8位'
					})
				}
			} else {
				this.$message.error({
					message: '未选择用户'
				})
			}
		}
	},

	mounted() {

	},

	computed: {
		...mapState(['global', 'user']),
	}
}
</script>

<style scoped>
p {
	color: #cfcfcf;
}
</style>
