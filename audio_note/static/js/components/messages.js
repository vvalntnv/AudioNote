import { removeParentElement } from "../utils.js";

export const applyCloseToMessages = () => {
	const closeButtons = document.getElementsByClassName('close-button');
	
	const animationTransitionTime = .4;
	const fadeOutAnimation = `fadeOut ${animationTransitionTime}s linear`;
	const animationDelayTime = 10 * 1000;

	for (let i = 0; i < closeButtons.length; i++) {
		const currentButton = closeButtons[i];

		currentButton.addEventListener('click', function() {	
			this.parentElement.style.animation = fadeOutAnimation;
			this.parentElement.addEventListener('animationend', removeParentElement);
		})

		setTimeout(() => {
			currentButton.parentElement.style.animation = fadeOutAnimation;	
			currentButton.parentElement.addEventListener('animationend', removeParentElement);
		}, animationDelayTime);
	}
}
