<template>
  <div @click="toggleDropdown" :class="['person', dropdown]">
    <h3>Dylan</h3>
    <div :class="['arrow-container', arrowClass]">
      <img src="../assets/down.svg" alt="" />
    </div>
  </div>
  <!-- <SingleRequest /> -->
</template>

<script>
export default {
  name: "SinglePerson",
  props: {
    name: String,
  },
  data() {
    return {
      isOpen: false,
    };
  },
  methods: {
    toggleDropdown() {
      console.log(this.isOpen);
      this.isOpen = !this.isOpen;
    },
  },
  computed: {
    arrowClass() {
      return this.isOpen ? "arrow-container--down" : "arrow-container--up ";
    },
    dropdown() {
      return this.isOpen ? "person--dropdown" : "person--up";
    },
  },
};
</script>

<style scoped>
h3 {
  margin-top: 2px;
  font-size: 24px;
  text-align: center;
}

.person {
  border-radius: 10px;
  padding: 10px;
  background-color: #f7f7ff;
  box-shadow: rgba(0, 0, 0, 0.24) 0px 3px 8px;
  position: relative;
  overflow: hidden;
}

.person--up {
  height: 50px;
  animation-name: goup;
  animation-duration: 400ms;
}

.person--dropdown {
  height: 400px;
  animation-name: dropdown;
  animation-duration: 400ms;
}

.arrow-container {
  position: absolute;
  right: 10px;
  top: 12.5px;
  transform: all 200ms;
}

.arrow-container--up {
  transition: 200ms;
  transform: translateY(0px) rotate(-0deg);
}

.arrow-container--down {
  transition: all 200ms;
  transform: translateY(-4px) rotate(180deg);
}

@keyframes dropdown {
  from {
    height: 50px;
  }
  to {
    height: 400px;
  }
}
@keyframes goup {
  from {
    height: 400px;
  }
  to {
    height: 50px;
  }
}
</style>
