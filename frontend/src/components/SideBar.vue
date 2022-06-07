<template>
  <div @click="debug" :class="['sidebar', slideOverClass]">
    <div class="spacer">&nbsp;</div>
    <RequestsContainer />
  </div>
</template>

<script>
import RequestsContainer from "./RequestsContainer.vue";
export default {
  name: "SideBar",
  components: { RequestsContainer },
  data() {
    return {
      slidOver: this.slideOver,
      windowWidth: window.innerWidth,
    };
  },
  props: {
    slideOver: Boolean,
  },
  computed: {
    slideOverClass() {
      if (this.windowWidth > 800) {
        return "";
      }
      return this.slidOver ? "slideover" : "slideover--off";
    },
  },
  watch: {
    slideOver() {
      this.slidOver = this.slideOver;
    },
  },
  mounted() {
    window.addEventListener("resize", () => {
      this.windowWidth = window.innerWidth;
    });
  },
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
  align-self: stretch;
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
    position: absolute;
  }
}

.slideover--off {
  width: 0px;
  top: 0;
  left: -200px;
  z-index: 2;
  animation-name: slideover--rev;
  animation-duration: 400ms;
  overflow: hidden;
}

.slideover {
  width: 100%;
  top: 0;
  left: 0;
  z-index: 2;
  animation-name: slideover;
  animation-duration: 400ms;
}

@keyframes slideover {
  from {
    width: 0px;
  }
  to {
    width: 100%;
  }
}

@keyframes slideover--rev {
  from {
    width: 100%;
  }
  to {
    width: 0px;
  }
}
</style>
