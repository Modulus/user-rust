<template>
  <div class="ui container">
    <h1>Welcome {{username}}</h1>
    {{ friends }}

    {{ messages }}
            
    

  </div>

</template>

<script>
export default {
  name: 'UsersComponent',
  mounted(){
    if (localStorage.jwtToken){
      this.jwtToken = localStorage.jwtToken
    }
    if(localStorage.username){
        this.username = localStorage.username
    }
  },  
  data() {
    return {
      friends: [],
      messages: []
    }
  },
  created() {
    this.getMessages()
    this.getFriends()
  },
  props: {
    msg: String
  },
  methods: {
   
      getMessages : function(){

      },
      getFriends: function(){

      }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h3 {
  margin: 40px 0 0;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}


</style>
