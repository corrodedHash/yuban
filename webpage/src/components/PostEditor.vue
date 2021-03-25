<template>
  <div>
    <post-diff
      v-if="orig_text !== undefined"
      :original="orig_text"
      :correction="text"
    ></post-diff>
    <textarea class="editor" v-model="text" :disabled="!canEdit"></textarea
    ><br />
    <el-button
      type="primary"
      icon="el-icon-check"
      @click="handleSubmit"
      :disabled="!canEdit"
      >Submit</el-button
    >
    <select ref="langcode">
      <option value="de">German</option>
      <option value="zh">Chinese</option>
    </select>
  </div>
</template>

<script lang="ts" src="./PostEditor.ts" />

<style scoped>
.editor {
  resize: none;
  width: 100%;
}
</style>
