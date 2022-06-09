<template>
  <div class="main">
    <h1 :style="{ color: h1Style }">Application</h1>
    <HamburgerIcon @toggled="toggled" :open="open" />
    <div class="content-container">
      <RotaContainer />
    </div>
  </div>
</template>

<script>
import RotaContainer from "./RotaContainer.vue";
import HamburgerIcon from "./HamburgerIcon.vue";
export default {
  name: "ContentContainer",
  components: { RotaContainer, HamburgerIcon },
  data() {
    return {
      open: false,
    };
  },
  methods: {
    toggled(toggle) {
      this.open = toggle;
    },
  },
  watch: {
    open() {
      this.$emit("opened", this.open);
    },
  },
  computed: {
    h1Style() {
      return this.open ? "#d3d0cb" : "#344055";
    },
  },
};
</script>

<style>
h1 {
  z-index: 50;
  transition: all 200ms;
}

.main {
  position: relative;
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

@media only screen and (max-width: 600px) {
  h1 {
    font-size: 30px;
  }
  h2 {
    font-size: 30px;
  }
  .main {
    padding: 20px 0px;
  }
}
</style>
