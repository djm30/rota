<template>
  <div class="request">
    <p class="request--task">Clean some fucking dishes</p>
    <div class="info">
      <span>03/06/22</span>
      <span>Dylan</span>
    </div>
  </div>
</template>

<script>
export default {
  name: "SingleRequest",
};
</script>

<style scoped>
.request {
  background-color: #f7f7ff;
  padding: 20px;
  font-size: 18px;
  box-shadow: rgba(0, 0, 0, 0.24) 0px 3px 8px;
  border-radius: 10px;
  margin-bottom: 16px;
}

.request--task {
  margin-bottom: 10px;
}

.info {
  display: flex;
  justify-content: space-between;
  color: #394851a9;
  font-size: 16px;
}
</style>
