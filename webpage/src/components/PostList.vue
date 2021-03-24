<template>
  <div>
    <el-button
      type="primary"
      icon="el-icon-plus"
      style="width: 100%"
      @click="selectNew"
      >Add Thread</el-button
    >
    <el-collapse ref="postList" accordion>
      <el-collapse-item
        v-for="thread in tableData"
        :key="thread.id"
        :name="thread.id"
      >
        <template #title> {{ thread.user }} - {{ thread.posttime }} </template>
        <el-button
          icon="el-icon-plus"
          style="width: 100%"
          @click="selectNewPost(thread.id)"
          >Add Language</el-button
        >
        <el-card
          shadow="hover"
          v-for="post in thread.posts"
          :key="post.id"
          class="postCard"
          @click="handleSelectedPost(thread.id, post)"
        >
          {{ post.user }} - {{ post.date }}<br />{{ post.text }}
        </el-card>
      </el-collapse-item>
    </el-collapse>
    <el-button @click="requestPosts">Get</el-button>
  </div>
</template>

<script lang="ts" src="./PostList.ts"/>

<style scoped>
.postCard {
  cursor: pointer;
}
.postCard:hover {
  cursor: pointer;
}
</style>
