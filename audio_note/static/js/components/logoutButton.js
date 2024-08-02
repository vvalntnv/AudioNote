const logoutUser = () => {
	fetch('/users/logout', {
		method: 'POST'
	})	
}
