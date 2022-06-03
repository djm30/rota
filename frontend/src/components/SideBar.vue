<template>
  <div class="sidebar">&nbsp;</div>
</template>

<script>
export default {
  name: "SideBar",
};
</script>

<style scoped>
.sidebar {
  width: 450px;
  background-color: #2e5266;
  flex-shrink: 1;
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
