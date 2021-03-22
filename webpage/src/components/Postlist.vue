<template>
  <div class="hello">
    <div> {{posts}}</div>
    <button @click="requestPosts">Get</button>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "Posts",
  data() {
    return { posts: "" };
  },
  methods: {
    requestPosts() {
      let postreq = new XMLHttpRequest();
      let me = this;
      postreq.onreadystatechange = (ev) => {
        if (postreq.readyState === 4 && postreq.status === 200) {
          me.posts = postreq.responseText;
        }
      };
      postreq.open("get", "/posts", true);
      postreq.send();
    },
  },
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h3 {
  margin: 40px 0 0;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}
</style>
