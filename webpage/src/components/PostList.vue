<template>
  <div>
    <el-button
      type="primary"
      icon="el-icon-plus"
      style="width: 100%"
      @click="selectNew"
      >Add Post</el-button
    >
    <el-table
      ref="postList"
      :data="tableData"
      highlight-current-row
      @current-change="handleSelectedPost"
    >
      <el-table-column prop="user" label="Name" />
      <el-table-column prop="text" label="Address" />
      <el-table-column prop="posttime" label="Date" />
    </el-table>
    <el-button @click="requestPosts">Get</el-button>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { get_posts, Post } from "./api";

export default defineComponent({
  name: "PostList",
  emits: {
    selectPost(id: number | null) {
      return true;
    },
  },
  data() {
    return {
      posts: [] as Post[],
    };
  },
  mounted() {
    this.requestPosts();
  },
  computed: {
    tableData(): Post[] {
      return this.posts;
    },
  },
  methods: {
    selectNew() {
      this.$emit("selectPost", null);
      (this.$refs.postList as any).setCurrentRow()
    },
    handleSelectedPost(index: Post) {
      if (index !== null) {
        console.log("Selected: ", index);
        this.$emit("selectPost", index.id);
      }
    },
    requestPosts() {
      let me = this;
      get_posts((req: XMLHttpRequest) => {
        if (req.readyState === 4 && req.status === 200) {
          me.posts = JSON.parse(req.responseText);
        }
      });
    },
  },
});
</script>

<style scoped>
</style>
