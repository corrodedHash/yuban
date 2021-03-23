<template>
  <el-container style="height: 500px; border: 1px solid #eee">
    <el-aside
      width="200px"
      style="background-color: rgb(238, 241, 246); overflow: none"
    >
      <post-list @select-post="handlePostSwitch" />
    </el-aside>

    <el-container>
      <el-header style="text-align: right; font-size: 12px">
        <el-button type="danger" @click="handleLogout" round>Logout</el-button>
      </el-header>

      <el-main>
        <router-view> </router-view>
      </el-main>
    </el-container>
  </el-container>
</template>
<script lang="ts" src="./PostWindow.ts" />
<style scoped>
</style>