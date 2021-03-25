<template>
  <el-container>
    <el-header style="text-align: right; font-size: 12px">
      <el-button @click="openList" round>Open list</el-button>
      <el-button type="danger" @click="handleLogout" round>Logout</el-button>
    </el-header>

    <el-main>
      <router-view> </router-view>
    </el-main>
  </el-container>
</template>
<script lang="ts" src="./PostWindow.ts" />
<style scoped>
</style>