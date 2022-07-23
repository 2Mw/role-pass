<template>
    <div>
        <el-table :data="passwords" id="table" height="470" stripe>
            <el-table-column type="expand">
                <template slot-scope="props">
                    <el-form label-position="left" inline class="demo-table-expand">
                        <el-form-item label="网站">
                            <span>{{ props.row.login_url }}</span>
                        </el-form-item>
                        <el-form-item label="账户">
                            <span>{{ props.row.account }}</span>
                        </el-form-item>
                        <el-form-item label="通行证">
                            <span>{{ props.row.password }}</span>
                        </el-form-item>
                        <el-form-item label="角色">
                            <span>{{ props.row.role }}</span>
                        </el-form-item>
                        <el-form-item label="修改次数">
                            <span>{{ props.row.update_count }}</span>
                        </el-form-item>
                        <el-form-item label="创建事件">
                            <span>{{ props.row.create_time }}</span>
                        </el-form-item>
                        <el-form-item label="上次修改">
                            <span>{{ props.row.last_update }}</span>
                        </el-form-item>
                        <el-form-item label="备注">
                            <span>{{ props.row.tip }}</span>
                        </el-form-item>
                    </el-form>
                </template>
            </el-table-column>
            <el-table-column sortable resizable v-for="(c, i) in showColumns" :key="i" :label="c.label" :prop="c.prop">
            </el-table-column>
            <el-table-column label="操作">
                <template slot-scope="scope">
                    <el-button size="mini" type="text" @click="handleEdit(scope.$index, scope.row)">编辑</el-button>
                    <!-- 删除按钮 -->
                    <el-popconfirm confirm-button-text='确认' cancel-button-text='取消' icon="el-icon-info"
                        icon-color="orange" :title="`确定删除 ` + scope.row.account + ` 吗？`" style="margin-left: 10px;"
                        @confirm="handleDelete(scope.$index, scope.row)" confirm-button-type="text">
                        <el-button size="mini" type="text" slot="reference">删除</el-button>
                    </el-popconfirm>

                </template>
            </el-table-column>
        </el-table>

        <!-- 修改账号 -->

        <el-dialog title="修改账号信息" :visible.sync="showModifyAccountDialog" width="40%" :close-on-click-modal="false"
            :center="true" top="5vh">
            <el-form label-position="left" label-width="80px">
                <el-form-item label="网址">
                    <el-input placeholder="" v-model="mLoginURL" :show-word-limit="true">
                    </el-input>
                </el-form-item>
                <el-form-item label="账号">
                    <el-input placeholder="" v-model="mAccount" :show-word-limit="true">
                    </el-input>
                </el-form-item>
                <el-form-item label="密码">
                    <el-input placeholder="" v-model="mAccountPassword" show-password :show-word-limit="true">
                    </el-input>
                </el-form-item>
                <el-form-item label="其他备注">
                    <el-input placeholder="" v-model="mTips" :show-word-limit="true" @keyup.enter.native="editAccount">
                    </el-input>
                </el-form-item>


            </el-form>
            <span slot="footer" class="dialog-footer">
                <el-button type="text" @click.native="editAccount" :loading="mAccountLoading">确 定
                </el-button>
            </span>
        </el-dialog>
    </div>
</template>

<script>
import { mapState } from 'vuex'
import moment from 'moment'
import { invoke } from '@tauri-apps/api';
import assert from 'assert';
import _ from 'lodash';

export default {
    name: 'PassTable',
    data() {
        return {
            showColumns: [
                { 'label': '网站', prop: 'login_url' },
                { 'label': '用户名', prop: 'account' },
                { 'label': '收录时间', prop: 'create_time' },
            ],
            popover_visiable: false,
            // modify account dialog
            showModifyAccountDialog: false,
            mAccountID: -1,
            mLoginURL: '',
            mAccount: '',
            mAccountPassword: '',
            mTips: '',
            mAccountLoading: false,
        }
    },
    computed: {
        ...mapState(['global', 'user']),
        passwords() {
            let cp = _.cloneDeep(this.user.passwords);
            let list = cp.filter(i => {
                i.create_time = this.toDate(i.create_time)
                i.last_update = this.toDate(i.last_update)
                return i.role == this.user.current_role;
            })
            let k = this.user.searchKey.toLowerCase();
            if (k.length > 0) {
                list = list.filter(i => {
                    return i.login_url.toLowerCase().indexOf(k) >= 0 
                    || i.account.toLowerCase().indexOf(k) >= 0
                    || i.tip.toLowerCase().indexOf(k) >= 0
                })
            }
            return list;
        }
    },

    methods: {
        toDate(ts) {
            return moment(ts.toString(), 'X').format('YYYY/MM/DD HH:mm:ss')
        },

        handleEdit(index, row) {
            index
            this.showModifyAccountDialog = true;
            this.mLoginURL = row.login_url;
            this.mAccount = row.account;
            this.mAccountPassword = row.password;
            this.mTips = row.tip;
            this.mAccountID = row.id
        },

        handleDelete(_index, row) {
            _index
            console.log("DELETE");
            invoke('del_accounts_by_id', { id: row.id })
                .then(rsp => {
                    if (rsp) {
                        // 删除成功
                        this.$store.dispatch('user/getUserRoles');
                        this.$store.dispatch('user/getUserAccounts');
                        assert(true, '删除失败(未知原因)')
                    } else {
                        assert(true, '删除失败(未知原因)')
                    }
                }).catch(e => {
                    this.$message.error({
                        message: e
                    })
                })
        },

        editAccount() {
            this.mAccountLoading = true;
            if (this.mAccount.length > 0) {
                if (this.mAccountPassword.length > 0) {
                    console.log(this.user.key);
                    invoke('update_accounts_by_id', {
                        id: this.mAccountID,
                        account: this.mAccount,
                        pass: this.mAccountPassword,
                        'loginUrl': this.mLoginURL,
                        tip: this.mTips,
                        upass: this.user.key,
                    }).then(resp => {
                        if (resp) {
                            this.$message({
                                message: '修改账号成功',
                                type: 'success'
                            });
                            this.mAccount = '';
                            this.mAccountPassword = '';
                            this.mLoginURL = '';
                            this.mTips = '';
                            this.mAccountID = -1;
                            this.$store.dispatch('user/getUserRoles');
                            this.$store.dispatch('user/getUserAccounts');
                            this.showModifyAccountDialog = false;
                        }
                    }).catch(e => {
                        this.$message.error({
                            message: e,
                        });
                    })
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
            this.mAccountLoading = false;
        }
    },

    mounted() {
        moment.locale('zh-cn')
    }
}
</script>

<style scoped>
#table {
    width: 100%;
}

.demo-table-expand {
    font-size: 0;
}

.demo-table-expand label {
    width: 90px;
    color: #99a9bf;
}

.demo-table-expand .el-form-item {
    margin-right: 0;
    margin-bottom: 0;
    width: 100%;
    padding: 0 40px;
}

.el-form-item span {
    user-select: text;
}
</style>