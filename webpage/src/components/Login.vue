<template>
  <div>
    Version 1
    <div
      v-loading="checkedToken"
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

<script lang="ts">
import { defineComponent } from "vue";
import { check_token, check_login } from "./api";
export default defineComponent({
  name: "Login",
  emits: {
    login() {
      return true;
    },
  },
  data() {
    return {
      username: "",
      password: "",
      checkedToken: false,
    };
  },
  mounted() {
    this.checkToken();
  },
  methods: {
    handleDoneRequest(req: XMLHttpRequest) {
      if (req.status === 202) {
        console.log("Accepted", req);
        this.$emit("login");
        this.checkedToken = false;
      } else {
        console.log("Not accepted", req);
        this.checkedToken = true;
      }
    },
    checkToken() {
      if (document.cookie.indexOf("token=") == -1) {
        console.log("No Token to test");
        return;
      }

      check_token((req: XMLHttpRequest) => {
        switch (req.readyState) {
          case 0:
          case 1:
          case 2:
          case 3:
            return;
          case 4:
            this.handleDoneRequest(req);
        }
      });
    },
    login() {
      let loginreq = new XMLHttpRequest();
      check_login(this.username, this.password, (req: XMLHttpRequest) => {
        switch (req.readyState) {
          case 0:
          case 1:
          case 2:
          case 3:
            console.log("Ready", req.readyState);
            return;
          case 4:
            this.handleDoneRequest(req);
        }
      });

      this.username = "";
      this.password = "";
    },
  },
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
