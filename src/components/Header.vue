<template>
    <div id="app">
        <el-row style="height: 28px;">
            <el-col :span="3" style="height: 100%; justify-content: center; align-items: center; display: flex;">
                <el-dropdown trigger="click">
                    <span class="el-dropdown-link">
                        选项<i class="el-icon-arrow-down el-icon--right"></i>
                    </span>
                    <el-dropdown-menu slot="dropdown">
                        <el-dropdown-item icon="el-icon-user" @click.native="selectUser">
                            选择用户</el-dropdown-item>
                        <el-dropdown-item icon="el-icon-plus" @click.native="showCreateUserDialog = true">添加用户
                        </el-dropdown-item>
                        <el-dropdown-item icon="el-icon-lock" @click.native="showSetAppPasswordDialog = true">修改登录密码
                        </el-dropdown-item>
                        <hr />
                        <el-dropdown-item icon="el-icon-upload2" @click.native="exportPasswords">导出</el-dropdown-item>
                        <el-dropdown-item icon="el-icon-download" disabled>导入</el-dropdown-item>
                        <hr />
                        <el-dropdown-item icon="el-icon-info" @click.native="showAppInfoDialog = true">程序信息
                        </el-dropdown-item>
                    </el-dropdown-menu>
                </el-dropdown>
            </el-col>
            <el-col :span="3" style="height: 100%; justify-content: center; align-items: center; display: flex;">
                <el-dropdown trigger="click">
                    <span class="el-dropdown-link">
                        用户<i class="el-icon-arrow-down el-icon--right"></i>
                    </span>
                    <el-dropdown-menu slot="dropdown">
                        <el-dropdown-item icon="el-icon-plus" @click.native="showAddAccountDialog = true">添加账号
                        </el-dropdown-item>
                        <el-dropdown-item icon="el-icon-edit">修改用户密码</el-dropdown-item>
                    </el-dropdown-menu>
                </el-dropdown>
            </el-col>

            <el-col :span="8" style="height: 100%;" :offset="2">
                <el-input placeholder="搜索" v-model="searchKey" size="mini" prefix-icon="el-icon-search"
                    :clearable="true" @input="handleInput"></el-input>
            </el-col>

            <el-col :span="3" :offset="5"
                style="height: 100%; justify-content: center; align-items: center; display: flex;">
                <el-dropdown trigger="click">
                    <span class="el-dropdown-link">
                        <i class="el-icon-user"> {{ user.current_user.name }}</i><i
                            class="el-icon-arrow-down el-icon--right"></i>
                    </span>
                    <el-dropdown-menu slot="dropdown">

                        <el-dropdown-item v-if="user.roles.length == 0" disabled>无角色</el-dropdown-item>
                        <el-dropdown-item v-for="(r, i) in user.roles" :key="i" @click.native="setRole(r.role)">{{
                                r.role
                        }}</el-dropdown-item>
                    </el-dropdown-menu>
                </el-dropdown>
            </el-col>
        </el-row>

        <!-- 修改 APP Lock -->
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

        <!-- 创建用户 -->
        <el-row>
            <el-col>
                <el-dialog title="创建用户" :visible.sync="showCreateUserDialog" width="40%" :close-on-click-modal="false"
                    :center="true" top="20vh">
                    <el-form label-position="left" label-width="80px">
                        <el-form-item label="用户名">
                            <el-input placeholder="" v-model="newUsername" :minlength="3" :show-word-limit="true">
                            </el-input>
                        </el-form-item>
                        <el-form-item label="密码">
                            <el-input placeholder="" v-model="newUserPassword" show-password :minlength="8"
                                :show-word-limit="true" @keyup.enter.native="createUser">
                            </el-input>
                        </el-form-item>
                    </el-form>
                    <span slot="footer" class="dialog-footer">
                        <el-button type="text" @click.native="createUser" :loading="createLoading">确 定</el-button>
                    </span>
                </el-dialog>
            </el-col>
        </el-row>

        <!-- 添加账号 -->
        <el-row>
            <el-col>
                <el-dialog title="添加账号" :visible.sync="showAddAccountDialog" width="40%" :close-on-click-modal="false"
                    :center="true" top="5vh">
                    <el-form label-position="left" label-width="80px">
                        <el-form-item label="所选角色">
                            <el-autocomplete class="inline-input" v-model="role" :fetch-suggestions="querySearch"
                                placeholder="请输入内容" style="width: 100%;"></el-autocomplete>
                        </el-form-item>
                        <el-form-item label="网址">
                            <el-input placeholder="" v-model="newLoginURL" :show-word-limit="true">
                            </el-input>
                        </el-form-item>
                        <el-form-item label="账号">
                            <el-input placeholder="" v-model="newAccount" :show-word-limit="true">
                            </el-input>
                        </el-form-item>
                        <el-form-item label="密码">
                            <el-input placeholder="" v-model="newAccountPassword" show-password :show-word-limit="true">
                            </el-input>
                        </el-form-item>
                        <el-form-item label="其他备注">
                            <el-input placeholder="" v-model="newTips" :show-word-limit="true"
                                @keyup.enter.native="createAccount">
                            </el-input>
                        </el-form-item>


                    </el-form>
                    <span slot="footer" class="dialog-footer">
                        <el-button type="text" @click.native="createAccount" :loading="createAccountLoading">确 定
                        </el-button>
                    </span>
                </el-dialog>
            </el-col>
        </el-row>

        <!-- 账号信息 -->
        <el-row>
            <el-col>
                <el-dialog title="Role-Pass" :visible.sync="showAppInfoDialog" width="50%" center :show-close="false"
                    style="user-select: auto;">
                    <p class="text">Role-Pass 为一款本地密码存储程序，由于采用单向哈希密码验证，用户密码请妥善保存，
                        否则将无法恢复，请即时<el-tooltip effect="dark" content="点击备份文件" placement="bottom">
                            <span class="slink" @click="exportPasswords" label="点击备份文件">备份</span>
                        </el-tooltip>密码文件。
                    </p>
                    <p class="text">Version: {{ global.version }}</p>
                    <p class="text">Release time: {{ global.releaseTime }}</p>
                </el-dialog>
            </el-col>
        </el-row>
    </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/tauri';
import { mapState } from 'vuex';
import { stringify } from 'csv-stringify';
import { writeTextFile, BaseDirectory } from '@tauri-apps/api/fs';

export default {
    name: "Header",
    data() {
        return {
            // App Lock dialog
            showSetAppPasswordDialog: false,
            oldPassword: '',
            newPassword: '',
            rePassword: '',
            resetLoading: false,
            // Create user dialog
            showCreateUserDialog: false,
            newUsername: '',
            newUserPassword: '',
            createLoading: false,
            // Add account dialog
            showAddAccountDialog: false,
            newAccount: '',
            newAccountPassword: '',
            role: '',
            newLoginURL: '',
            newTips: '',
            createAccountLoading: false,
            // App info dialog
            showAppInfoDialog: false,
            // Search key
            searchKey: '',
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
        },

        createUser() {
            this.createLoading = true
            if (this.newUsername.length >= 3) {
                if (this.newUserPassword.length >= 8) {
                    invoke('create_user', { 'name': this.newUsername, 'pass': this.newUserPassword })
                        .then(rsp => {
                            if (rsp) {
                                this.$message({
                                    message: `创建用户 ${this.newUsername} 成功`,
                                    type: 'success'
                                });
                                this.$store.dispatch('user/queryUsers');
                                this.newUserPassword = '';
                                this.newUsername = '';
                                this.showCreateUserDialog = false
                            } else {
                                this.$message.error({
                                    message: '创建失败',
                                });
                            }
                        }).catch(e => {
                            this.$message.error({
                                message: e,
                            });
                        })
                } else {
                    this.$message({
                        message: '密码长度需不小于8位',
                        type: 'warning'
                    });
                }
            } else {
                this.$message({
                    message: '用户名长度需不小于3位',
                    type: 'warning'
                });
            }
            this.createLoading = false
        },

        querySearch(queryString, cb) {
            let results = queryString ? this.rolesList.filter(i => {
                return i.value.toLowerCase().indexOf(queryString.toLowerCase()) === 0;
            }) : this.rolesList;
            // 调用 callback 返回建议列表的数据
            cb(results);
        },

        createAccount() {
            this.createAccountLoading = true;
            if (this.newAccount.length > 0) {
                if (this.newAccountPassword.length > 0) {
                    if (this.role.length > 0) {
                        // console.log(this.user.key);
                        invoke('insert_account', {
                            uid: this.user.current_user.id,
                            role: this.role,
                            account: this.newAccount,
                            pass: this.newAccountPassword,
                            'loginUrl': this.newLoginURL,
                            tip: this.newTips,
                            upass: this.user.key,
                        }).then(resp => {
                            if (resp) {
                                this.$message({
                                    message: '添加账号成功',
                                    type: 'success'
                                });
                                this.newAccount = '';
                                this.newAccountPassword = '';
                                this.newLoginURL = '';
                                this.newTips = "";
                                this.$store.dispatch('user/getUserRoles');
                                this.$store.dispatch('user/getUserAccounts');
                            }
                        }).catch(e => {
                            this.$message.error({
                                message: e,
                            });
                        })
                    } else {
                        this.$message({
                            message: '需要选取角色',
                            type: 'warning'
                        });
                    }
                } else {
                    this.$message({
                        message: '密码不能为空',
                        type: 'warning'
                    });
                }
            } else {
                this.$message({
                    message: '用户名不能为空',
                    type: 'warning'
                });
            }
            this.createAccountLoading = false;
        },

        setRole(r) {
            this.$store.dispatch('user/setCurrentRole', r)
        },

        handleInput(v) {
            this.$store.dispatch('user/setSearchKey', v)
        },

        exportPasswords() {
            let _this = this;
            stringify(this.user.passwords, { header: true }, function (err, data) {
                console.error(err);
                console.log(data);
                writeTextFile({ path: `${_this.user.current_user.name}_export.csv`, contents: data }, { dir: BaseDirectory.Desktop })
                    .then(_ => {
                        _
                        _this.$message({
                            message: '导出成功，已保存到桌面',
                            type: 'success'
                        });
                    }).catch(e => {
                        _this.$message.error({
                            message: '导出失败：' + e,
                        });
                    })
            })
        }
    },
    computed: {
        ...mapState(['global', 'user']),
        rolesList() {
            let arr = [{ 'value': 'main' }];
            if (this.user.roles.length > 0) arr = [];
            this.user.roles.forEach(e => {
                arr.push({ 'value': e.role });
            });
            return arr;
        }
    },

    mounted() {
        let i = setInterval(() => {
            if (this.user.users.length === 0) {
                if (!this.showCreateUserDialog) {
                    this.showCreateUserDialog = true;
                    this.$message({
                        message: '请先创建用户',
                        type: 'info'
                    });
                }

            } else {
                clearInterval(i);
                this.showCreateUserDialog = false;
            }
        }, 1000);
    }
}
</script>

<style scoped>
#app {
    padding: 10px;
    color: #cfcfcf;
    border-bottom: 1px solid #797979;
}

.text {
    margin-top: 10px;
    color: #cfcfcf;
    user-select: text;
}

.slink {
    cursor: pointer;
}

.slink:hover {
    border-bottom: 1px dashed #cfcfcf;
}
</style>