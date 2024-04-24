export default function Loader() {

	return <span id="loader">
		<style>
			{`
				#loader {
					width: 12px;
					height: 12px;
					border-radius: 50%;
					display: block;
					margin:15px auto;
					position: relative;
					color: #FFF;
					box-sizing: border-box;
					animation: animloader 2s linear infinite;
				}
				
				@keyframes animloader {
					0% {
						box-shadow: 14px 0 0 -2px,  38px 0 0 -2px,  -14px 0 0 -2px,  -38px 0 0 -2px;
					}
					25% {
						box-shadow: 14px 0 0 -2px,  38px 0 0 -2px,  -14px 0 0 -2px,  -38px 0 0 2px;
					}
					50% {
						box-shadow: 14px 0 0 -2px,  38px 0 0 -2px,  -14px 0 0 2px,  -38px 0 0 -2px;
					}
					75% {
						box-shadow: 14px 0 0 2px,  38px 0 0 -2px,  -14px 0 0 -2px,  -38px 0 0 -2px;
					}
					100% {
						box-shadow: 14px 0 0 -2px,  38px 0 0 2px,  -14px 0 0 -2px,  -38px 0 0 -2px;
					}
				}
			`}
		</style>
	</span>
}