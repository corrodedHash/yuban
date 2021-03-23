<template>
  <div>
    <div v-if="!loggedIn">
      <login @login="loggedIn = true" />
    </div>
    <div v-else>
      <post-window @logout="loggedIn = false" />
    </div>
  </div>
</template>

<script lang="ts" src="./App.ts" />
<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
