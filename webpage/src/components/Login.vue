<template>
  <div>
    Version 1
    <form action="logindata" method="POST" @submit.prevent="login">
      <input type="text" name="username" v-model="username" /><br />
      <input type="password" name="password" v-model="password" /><br />
      <input type="submit" value="Submit" />
    </form>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

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
      } else {
        console.log("Not accepted", req);
      }
    },
    checkToken() {
      if (document.cookie.indexOf("token=") == -1) {
        console.log("No Token to test")
        return;
      }
      let loginreq = new XMLHttpRequest();
      loginreq.onreadystatechange = (ev) => {
        switch (loginreq.readyState) {
          case 0:
          case 1:
          case 2:
          case 3:
            return;
          case 4:
            this.handleDoneRequest(loginreq);
        }
      };
      loginreq.open("GET", "/testtoken", true);
      loginreq.send();
    },
    login() {
      let loginreq = new XMLHttpRequest();
      loginreq.onreadystatechange = (ev) => {
        switch (loginreq.readyState) {
          case 0:
          case 1:
          case 2:
          case 3:
            return;
          case 4:
            this.handleDoneRequest(loginreq);
        }
      };
      let json_data = JSON.stringify({
        username: this.username,
        password: this.password,
      });
      loginreq.open("POST", "/logindata", true);
      loginreq.send(json_data);
      console.log("Sending", json_data);

      this.username = "";
      this.password = "";
    },
  },
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
