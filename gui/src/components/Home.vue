<template>
  <div class="ui container">
    <h1>{{ msg }}</h1>
    {{ username }}
    {{ password }}

    <form class="ui large form">
      <div class="ui  two fields">
        <div class="field">
          <input class="ui left input" v-model="username" type="text" placeholder="Username"/>
        </div>
        <div class="field">
          <input class="ui left input" v-model="password" type="password" placeholder="Password"/>
        </div>   
      </div>
      <div class=" ">
        <button v-on:click="login()" class="ui button pink left floated">Login</button>  
        <button v-on:click="register()" class="ui button green left floated">Register</button>
      </div> 
            
    
    </form>

  </div>

</template>

<script>
export default {
  name: 'Home',
  data() {
    return {
      username: "",
      password: "",
      newUser: {}
    }
  },
  props: {
    msg: String
  },
  methods: {
    register: function(){
      console.log(this.username)
      console.log(this.password)
      fetch("http://localhost:8080/users/add", {
          headers: { "Content-Type": "application/json" },
          method: 'POST',
          // redirect: "follow",
          // cache: "no-cache",
          // mode: "no-cors", //no-cors, same-origin
          body: JSON.stringify({
            name: this.username,
            password: this.password,
            active: true,
            comment: "New user"
          })
        }).then(response => {
          console.log(response)
          return response.json()
        })
        .then(data => {
          console.log("Data: ", data)
          this.newUser = data
        })
        .catch((e) =>  {
            console.log("Failed", e)
        })
    },
    login: function(){
      alert("Attembing to login!")
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
