<template>
  <div>
    <textarea class="editor" v-model="text"></textarea><br />
    <el-button type="primary" icon="el-icon-check" @click="handleSubmit"
      >Submit</el-button
    >
  </div>
</template>

<script lang="ts" src="./PostEditor.ts" />

<style scoped>
.editor {
  resize: none;
  width: 100%;
}
</style>
