import { getCookie } from "../utils.js";


export const logoutUser = async () => {
	const csrfToken = getCookie("csrftoken");

	const response = await fetch('/users/logout', {
		method: 'POST',
		headers: {
			'X-CSRFToken': csrfToken
		}
	})	

	if (response.redirected) {
		window.location.href = response.url;
	}
}
