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
        <template #title>
          {{ thread.user }} - {{ thread.posttime.toLocaleString() }}
        </template>
        <el-button
          icon="el-icon-plus"
          style="width: 100%"
          @click="selectNewPost(thread.id)"
          >Add Language</el-button
        >
        <div
          v-for="post in thread.posts"
          :key="post.id"
          class="postCard"
          @click="handleSelectedPost(thread.id, post)"
        >
          {{ post.user }} - {{ post.date.toLocaleString() }} - {{ post.lang
          }}<br />{{ post.ellipsis }}...
          <br />
          <el-button
            icon="el-icon-plus"
            size="mini"
            @click.stop="selectNewCorrection(thread.id, post)"
            >Add Correction</el-button
          >
          <div
            class="correctionBox"
            v-for="corr in post.corrections"
            :key="corr.id"
            @click.stop="handleSelectedCorrection(thread.id, post, corr.id)"
          >
            {{ corr.username }} - {{ corr.postdate.toLocaleString() }}
          </div>
        </div>
      </el-collapse-item>
    </el-collapse>
    <el-button @click="requestPosts">Get</el-button>
  </div>
</template>

<script lang="ts" src="./PostList.ts"/>

<style scoped>
.postCard {
  cursor: pointer;
  border: 1px;
  border-style: dotted;
  border-radius: 4px;
  margin: 3px;
  padding: 3px;
  background-color: rgb(238, 244, 247);
}
.postCard:hover {
  cursor: pointer;
  box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.1);
  border-style: solid;
  background-color: rgb(142, 187, 204);
}
</style>
