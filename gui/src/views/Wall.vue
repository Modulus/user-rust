<template>
  <div class="home">
    <!-- <img alt="Vue logo" src="../assets/logo.png"> -->
    <WallComponent />
  </div>
</template>

<script>
// @ is an alias to /src
import WallComponent from '@/components/WallComponent.vue'

export default {
  name: 'Wall',
  components: {
    WallComponent
  }
}
</script>
