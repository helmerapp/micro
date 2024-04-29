import { ReactElement, useEffect, useState } from "react";
import { PostHogProvider, usePostHog } from "posthog-js/react"
import { invoke } from "@tauri-apps/api/core";
import { app } from "@tauri-apps/api";


const posthogKey = import.meta.env.PUBLIC_POSTHOG_KEY;
const posthogHost = import.meta.env.PUBLIC_POSTHOG_HOST;

export default function PosthogProvider({ children }: { children: ReactElement }) {

	const [isOkSharingUsageData, setIsOkSharingUsageData] = useState(true);
	const posthog = usePostHog();

	useEffect(() => {
		app.getVersion().then(version => {
			posthog.register({
				'App Version': version
			})
		})
	}, [])

	useEffect(() => {
		invoke('is_ok_sharing_usage_data').then((res) => {
			console.log("isOkSharingUsageData", res);
			setIsOkSharingUsageData(res as boolean);
		})
	}, [])

	if (!isOkSharingUsageData) {
		return <>{children}</>
	}

	return <PostHogProvider
		apiKey={posthogKey}
		options={{
			api_host: posthogHost,
			api_transport: "fetch"
		}}>
		{children}
	</PostHogProvider>
}