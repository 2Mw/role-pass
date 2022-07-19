<template>
    <div id="app">
        <el-row>
            <el-col :span="2">
                <el-dropdown trigger="click">
                    <span class="el-dropdown-link">
                        菜單1<i class="el-icon-arrow-down el-icon--right"></i>
                    </span>
                    <el-dropdown-menu slot="dropdown">
                        <el-dropdown-item icon="el-icon-plus">黄金糕</el-dropdown-item>
                        <el-dropdown-item icon="el-icon-circle-plus">狮子头</el-dropdown-item>
                    </el-dropdown-menu>
                </el-dropdown>
            </el-col>
            <el-col :span="2">
                <el-dropdown trigger="click">
                    <span class="el-dropdown-link">
                        菜單2<i class="el-icon-arrow-down el-icon--right"></i>
                    </span>
                    <el-dropdown-menu slot="dropdown">
                        <el-dropdown-item icon="el-icon-plus">黄金糕</el-dropdown-item>
                        <el-dropdown-item icon="el-icon-circle-plus">狮子头</el-dropdown-item>
                    </el-dropdown-menu>
                </el-dropdown>
            </el-col>
            <el-col :span="2">
                <el-dropdown trigger="click">
                    <span class="el-dropdown-link">
                        Roles<i class="el-icon-arrow-down el-icon--right"></i>
                    </span>
                    <el-dropdown-menu slot="dropdown">
                        <el-dropdown-item icon="el-icon-user">Role - 1</el-dropdown-item>
                        <el-dropdown-item icon="el-icon-user">Role - 2</el-dropdown-item>
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
    </div>
</template>

<script>
export default {
    name: "Header",
    data() {
        return {}
    },
}
</script>

<style scoped>
#app {
    padding: 10px;
    color: #cfcfcf;
    border-bottom: 1px solid #797979;
}
</style>