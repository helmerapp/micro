export default function Loader() {
	return <span id="loader" className="w-3 h-3 rounded-full block mx-auto mt-4 mb-4 relative text-white box-border">
		<style>
			{`
				#loader {
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