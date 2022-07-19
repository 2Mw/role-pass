<template>
  <div class="home">
    <Header/>
    <br/>
    <h1>User - Role</h1>
    <br/>
    <hr>
    <PassTable/>
  </div>
</template>

<script>
import Header from '../components/Header.vue';
import PassTable from '../components/PassTable.vue';
// @ is an alias to /src
// import HelloWorld from '@/components/HelloWorld.vue'

export default {
  name: 'Home',
  components: {
    Header,
    PassTable
}
}
</script>
