<template>
  <div>
    Version 1
    <div
      v-loading="!checkedToken"
      element-loading-text="Loading..."
      element-loading-spinner="el-icon-loading"
    >
      <form action="logindata" method="POST" @submit.prevent="login">
        <input type="text" name="username" v-model="username" /><br />
        <input type="password" name="password" v-model="password" /><br />
        <input type="submit" value="Submit" />
      </form>
    </div>
  </div>
</template>

<script lang="ts" src="./Login.ts"/>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
