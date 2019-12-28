Vue.component('hex', {
	data: function(){
		return {
			tokens: 0
		}
	},
	template: '<button v-on:click="tokens++">{{tokens}}</button>'
})
