// decodeURIComponent(atob(btoa(encodeURIcomponent("string"))))
export type Duration = {
	secs: number;
	nanos: number;
};

export type Cover = {
	mime_type: string | null;
	data: string | null;
};

export type Album = {
	title: string | null;
	artist: string | null;
	cover: Cover | null;
	tracks: number | null;
	year: number | null;
};

export type Track = {
	artist: string | null;
	title: string | null;
	len: Duration | null;
	album: Album | null;
	genre: string | null;
	track: number | null;
	source: string;
};

export type State = {
	current: Current | null;
	next: Track | null;
	prev: Track | null;
};

export type Current = {
	song: Track;
	paused: boolean;
};

export type Library = {
	position: Duration;
	current: Current | null;
	cover: string;
	next: Track | null;
	prev: Track | null;
	songs: [number, Track][];
	ui: {
		showActions: boolean;
		timePretty: (secs: number) => string;
		positionPercent: (current: number, duration: number | null) => number;
		base64ToFile: (source: string) => string;
	};
};
export let library = $state<Library>({
	current: null,
	position: {
		secs: 0,
		nanos: 0,
	},
	cover: "",
	next: null,
	prev: null,
	songs: [],
	ui: {
		showActions: false,
		timePretty: (secs: number) => {
			let date = new Date(0);
			date.setSeconds(secs);
			if (secs > 3600) {
				return date.toISOString().substring(11, 19);
			} else {
				return date.toISOString().substring(14, 19);
			}
		},
		positionPercent: (secs: number, duration: number | null) => {
			if (duration) {
				return (secs / duration) * 100;
			}
			return 0;
		},
		base64ToFile: (source: string) => {
			const base64Data = source.replace(/^data:.+;base64,/, "");
			const byteCharacters = atob(base64Data);
			const byteNumbers = new Array(byteCharacters.length);
			for (let i = 0; i < byteCharacters.length; i++) {
				byteNumbers[i] = byteCharacters.charCodeAt(i);
			}
			const byteArray = new Uint8Array(byteNumbers);
			const blob = new Blob([byteArray], { type: "image/jpeg" });
			const url = URL.createObjectURL(blob);
			return url;
		},
	},
});
