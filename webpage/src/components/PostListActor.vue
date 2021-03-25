<template>
  <post-list
    @select-post="handlePostSwitch"
    @select-correction="handleCorrectionSwitch"
  />
</template>

<script lang="ts" >
import { defineComponent } from "vue";
import PostList from "./PostList.vue";
export default defineComponent({
  name: "PostListActor",
  components: { PostList },
  methods: {
    handleCorrectionSwitch(
      thread_id: number,
      post_id: number,
      correction: number | null
    ) {
      if (correction !== null) {
        this.$router.push({
          name: "Correction",
          params: { postid: post_id, corrid: correction },
        });
      } else {
        this.$router.push({
          name: "NewCorrection",
          params: { postid: post_id },
        });
      }
    },
    handlePostSwitch(thread_id: number | null, post_id: number | null) {
      if (post_id === null && thread_id === null) {
        this.$router.push({ path: "/" });
      } else if (post_id !== null && thread_id === null) {
      } else if (post_id !== null && thread_id !== null) {
        this.$router.push({ name: "View", params: { postid: post_id } });
      } else if (post_id === null && thread_id !== null) {
        this.$router.push({ name: "NewPost", params: { threadid: thread_id } });
      }
    },
  },
});
</script>

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
