<template>
  <div @click="toggle" :class="'hamburger-container ' + openClass">
    <span class="ham-top ham-line"></span>
    <span class="ham-middle ham-line"></span>
    <span class="ham-bottom ham-line"></span>
  </div>
</template>

<script>
export default {
  props: {
    open: Boolean,
  },
  data() {
    return {
      isOpen: this.open,
    };
  },
  methods: {
    toggle() {
      this.$emit("toggled", !this.open);
    },
  },
  computed: {
    openClass() {
      return this.isOpen ? "open" : "";
    },
  },
  emits: ["toggled"],
  watch: {
    open() {
      this.isOpen = this.open;
    },
  },
};
</script>

<style scoped>
.hamburger-container {
  position: absolute;
  width: 24px;
  height: 24px;
  cursor: pointer;
  transition: all 0.25s;
  top: 40px;
  left: 20px;
  display: none;
  z-index: 30;
}

.ham-line {
  position: absolute;
  top: 0;
  left: 0;
  background: #344055;
  width: 24px;
  height: 2px;
  transform: rotate(0);
  transition: all 0.5s;
}
.brown-town {
  background-color: #5c3e09;
}

.ham-middle {
  transform: translateY(7px);
}

.ham-bottom {
  transform: translateY(14px);
}

.open .ham-line {
  transform: rotate(90deg);
  transform: translateY(0px);
  background-color: #d3d0cb;
}

.open .ham-top {
  transform: rotate(45deg) translateY(6px) translate(6px);
}

.open .ham-middle {
  display: none;
}

.open .ham-bottom {
  transform: rotate(-45deg) translateY(6px) translate(-6px);
}

@media only screen and (max-width: 900px) {
  .hamburger-container {
    display: block;
  }
}
</style>
