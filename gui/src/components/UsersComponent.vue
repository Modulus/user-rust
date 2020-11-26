<template>
  <div class="ui container">
    <h1>{{ msg }}</h1>
    {{ users }}

 
            
    

  </div>

</template>

<script>
export default {
  name: 'UsersComponent',
  data() {
    return {
      users: []
    }
  },
  created() {
    this.getUsers()
  },
  props: {
    msg: String
  },
  methods: {
    getUsers: function(){
      fetch("http://localhost:8080/users")
        .then(stream => stream.json())
        .then(data => {
          this.users = data;
          this.error = null
          console.log("Created new name at servetime: " + this.data)
        })
        .catch(error => {
          console.error(error)
          this.error = error
          console.log("Error: ", error)
        })
  
    },
    register: function(){
      console.log(this.username)
      console.log(this.password)
      fetch("http://localhost:8080/users/add", {
          headers: { "Content-Type": "application/json; UTF-8" },
          method: 'POST',
          redirect: "follow",
          cache: "no-cache",
          mode: "no-cors", //no-cors, same-origin
          body: JSON.stringify({
            name: this.username,
            password: this.password,
            active: true,
            comment: "New user"
          })
        }).then(response => console.log("Jadda", response))
        .then(console.log("Hmm"))
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
