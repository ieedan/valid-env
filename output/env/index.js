import priv from "./private.js";
import pub from "./public.js";

/** @type {import('.').env} */
const env = {
	private: priv,
	public: pub
}

export default env;