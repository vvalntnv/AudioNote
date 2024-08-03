export function getCookie(cookieName) {
	const name = cookieName + '=';
	const decodedCookie = decodeURIComponent(document.cookie)
	const cookies = decodedCookie.split(';');
	for (let i = 0; i < cookies.length; i++) {
		const currentCookie = cookies[i].trim();

		if (currentCookie.substring(0, name.length) === name) {
			return currentCookie.substring(name.length);
		} 
	}

	return null;
}
