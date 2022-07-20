<template>
    <div id="app">
        <el-row>
            <el-col :span="3">
                <el-dropdown trigger="click">
                    <span class="el-dropdown-link">
                        选项<i class="el-icon-arrow-down el-icon--right"></i>
                    </span>
                    <el-dropdown-menu slot="dropdown">
                        <el-dropdown-item icon="el-icon-user" @click.native="selectUser">
                            选择用户</el-dropdown-item>
                        <el-dropdown-item icon="el-icon-plus">添加用户</el-dropdown-item>
                        <el-dropdown-item icon="el-icon-lock" @click.native="showSetAppPasswordDialog = true">修改登录密码
                        </el-dropdown-item>
                        <hr />
                        <el-dropdown-item icon="el-icon-upload2" disabled>导出</el-dropdown-item>
                        <el-dropdown-item icon="el-icon-download" disabled>导入</el-dropdown-item>
                    </el-dropdown-menu>
                </el-dropdown>
            </el-col>
            <el-col :span="3">
                <el-dropdown trigger="click">
                    <span class="el-dropdown-link">
                        用户<i class="el-icon-arrow-down el-icon--right"></i>
                    </span>
                    <el-dropdown-menu slot="dropdown">
                        <el-dropdown-item icon="el-icon-plus">添加账号</el-dropdown-item>
                        <el-dropdown-item icon="el-icon-edit">修改用户密码</el-dropdown-item>
                    </el-dropdown-menu>
                </el-dropdown>
            </el-col>

            <el-col :span="3" :offset="15">
                <el-dropdown trigger="click">
                    <span class="el-dropdown-link">
                        <i class="el-icon-user"> Username</i><i class="el-icon-arrow-down el-icon--right"></i>
                    </span>
                    <el-dropdown-menu slot="dropdown">
                        <el-dropdown-item>Role - 1</el-dropdown-item>
                        <el-dropdown-item>Role - 2</el-dropdown-item>
                    </el-dropdown-menu>
                </el-dropdown>
            </el-col>
        </el-row>
        <el-row>
            <el-col>
                <el-dialog title="修改程序锁密码" :visible.sync="showSetAppPasswordDialog" width="50%"
                    :close-on-click-modal="false" :center="true" top="20vh">
                    <el-input placeholder="旧密码" v-model="oldPassword" show-password :minlength="8"
                        :show-word-limit="true" style="margin-top: 10px">
                    </el-input>
                    <el-input placeholder="新密码" v-model="newPassword" show-password :minlength="8"
                        :show-word-limit="true" style="margin-top: 10px">
                    </el-input>
                    <el-input placeholder="重新输入新密码" v-model="rePassword" show-password :minlength="8"
                        :show-word-limit="true" style="margin-top: 10px" @keyup.enter.native="resetPassword">
                    </el-input>
                    <span slot="footer" class="dialog-footer">
                        <el-button type="text" @click="resetPassword" :loading="resetLoading">确 定</el-button>
                    </span>
                </el-dialog>
            </el-col>
        </el-row>
    </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/tauri';
import { mapState } from 'vuex';

export default {
    name: "Header",
    data() {
        return {
            showSetAppPasswordDialog: false,
            oldPassword: '',
            newPassword: '',
            rePassword: '',
            resetLoading: false,
        }
    },
    methods: {
        resetPassword() {
            this.resetLoading = true;
            if (this.newPassword.length < 8 || this.oldPassword.length < 8) {
                this.$message.error({
                    message: '密码长度需大于8位',
                });
                this.resetLoading = false;
                return;
            }
            if (this.newPassword !== this.rePassword) {
                this.$message.error({
                    message: '新密码输入不一致',
                });
                this.resetLoading = false;
                return;
            }

            invoke('valid_app_password', { 'pass': this.oldPassword }).then(resp => {
                if (resp) {
                    invoke('set_app_password', { 'pass': this.newPassword }).then(resp => {
                        if (resp) {
                            this.$message({
                                message: '修改成功',
                                type: 'success'
                            });
                            this.oldPassword = "";
                            this.newPassword = "";
                            this.rePassword = "";
                            setTimeout(() => {
                                this.showSetAppPasswordDialog = false
                            }, 200)
                        }
                        else this.$message.error({
                            message: '修改失败',
                        });
                        this.resetLoading = false;
                    })
                }
                else this.$message.error({
                    message: '旧密码错误',
                });
                this.resetLoading = false;
            })

        },

        selectUser() {
            this.$store.dispatch('global/exitUserLogin')
        }
    },
    computed: {
        ...mapState(['global', 'user']),
    }
}
</script>

<style scoped>
#app {
    padding: 10px;
    color: #cfcfcf;
    border-bottom: 1px solid #797979;
}
</style>