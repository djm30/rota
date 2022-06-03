<template>
  <div class="requests">
    <h2>Requests</h2>
    <div class="requests-container">
      <SingleRequest />
      <SingleRequest />
      <SingleRequest />
      <SingleRequest />
      <SingleRequest />
      <SingleRequest />
      <SingleRequest />
      <SingleRequest />
    </div>
  </div>
</template>

<script>
import SingleRequest from "./SingleRequest.vue";
export default {
  name: "RequestsContainer",
  components: { SingleRequest },
};
</script>

<style scoped>
.requests {
  width: 80%;
  display: flex;
  align-items: center;
  flex-direction: column;
}

h2 {
  font-size: 39px;
  color: #d3d0cb;
}
.requests-container {
  padding-top: 24px;
  width: 100%;
}

@media only screen and (max-width: 800px) {
  .requests {
    width: 90%;
  }
}
</style>
