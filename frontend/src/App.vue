<template>
  <div class="wrapper">
    <SideBar />
    <ContentContainer />
  </div>
</template>

<script>
import SideBar from "./components/SideBar.vue";
import ContentContainer from "./components/ContentContainer.vue";

export default {
  name: "App",
  components: {
    SideBar,
    ContentContainer,
  },
};
</script>

<style>
.wrapper {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  min-height: 100vh;
  display: flex;
  width: 100%;
  display: flex;
}
</style>
