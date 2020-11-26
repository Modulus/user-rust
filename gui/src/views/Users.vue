<template>
  <div class="home">
    <!-- <img alt="Vue logo" src="../assets/logo.png"> -->
    <UsersComponent msg="Users"/>
  </div>
</template>

<script>
// @ is an alias to /src
import UsersComponent from '@/components/UsersComponent.vue'

export default {
  name: 'Users',
  components: {
    UsersComponent
  }
}
</script>
