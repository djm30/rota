<template>
  <div class="rota">
    <h2>Rota</h2>
    <div class="rota--container">
      <SinglePerson />
      <SinglePerson />
      <SinglePerson />
      <SinglePerson />
    </div>
  </div>
</template>

<script>
import SinglePerson from "./SinglePerson.vue";
export default {
  name: "RotaContainer",
  components: { SinglePerson },
};
</script>

<style scoped>
.rota {
  width: 67%;
  display: flex;
  flex-direction: column;
  align-items: center;
  flex-grow: 1;
}
.rota--container {
  padding-top: 24px;
  display: grid;
  grid-template-columns: repeat(1, 1fr);
  width: 70%;
  gap: 20px;
}

@media only screen and (max-width: 800px) {
  .rota {
    width: 90%;
  }
  .rota--container {
    grid-template-columns: repeat(1, 1fr);
  }
}
</style>
