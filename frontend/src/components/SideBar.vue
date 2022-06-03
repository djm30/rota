<template>
  <div class="sidebar">
    <div class="spacer">&nbsp;</div>
    <RequestsContainer />
  </div>
</template>

<script>
import RequestsContainer from "./RequestsContainer.vue";
export default {
  name: "SideBar",
  components: { RequestsContainer },
};
</script>

<style scoped>
.sidebar {
  padding: 20px;
  width: 450px;
  background-color: #2e5266;
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  align-items: center;
  user-select: none;
}

.spacer {
  font-size: 52px;
  margin-bottom: 36px;
}

@media only screen and (max-width: 1180px) {
  .sidebar {
    flex-shrink: 6;
  }
}

@media only screen and (max-width: 1000px) {
  .sidebar {
    flex-shrink: 10;
  }
}

@media only screen and (max-width: 900px) {
  .sidebar {
    display: none;
  }
}
</style>
