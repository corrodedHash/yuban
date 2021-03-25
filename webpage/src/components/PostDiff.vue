<template>
  <div class="diffbox">
    <span v-for="d in difftext" :key="d.id" :class="d.type">{{ d.text }}</span>
  </div>
</template>

<script lang="ts" src="./PostDiff.ts" />

<style scoped>
.diffbox {
  margin-bottom: 10px;
}
.normal {
}
.add {
  /* color: greenyellow; */
  background-color: rgba(64, 187, 64, 0.4);
  padding: 3px 5px 3px 5px;
  border: 1px;
  border-style: solid;
  border-radius: 30px;
}
.sub {
  /* color: lightcoral; */
  background-color: rgba(212, 74, 74, 0.4);
  padding: 3px 5px 3px 5px;
  border: 1px;
  border-style: solid;
  border-radius: 30px;
}
</style>
