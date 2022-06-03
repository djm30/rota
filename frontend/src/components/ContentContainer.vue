<template>
  <div class="main">
    <h1>Application</h1>
    <div class="content-container">
      <RequestsContainer />
      <RotaContainer />
    </div>
  </div>
</template>

<script>
import RequestsContainer from "./RequestsContainer.vue";
import RotaContainer from "./RotaContainer.vue";
export default {
  name: "ContentContainer",
  components: { RequestsContainer, RotaContainer },
};
</script>

<style>
.main {
  width: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 20px 80px;
  color: #344055;
  font-size: 26px;
  flex-shrink: 1;
  background-color: #d3d0cb;
}

.content-container {
  display: flex;
  width: 100%;
  margin-top: 36px;
  gap: 32px;
}

@media only screen and (max-width: 940px) {
  .main {
    flex-shrink: 0;
    padding: 20px 40px;
  }
}

@media only screen and (max-width: 800px) {
  .content-container {
    flex-direction: column;
    align-items: center;
  }
}
</style>
