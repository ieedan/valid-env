export type env = {
	private: Private;
	public: Public;
};

export type Public = {
	NOTHING: string
}

export type Private = {
	SOMETHING: string,
	ANYTHING: string,
	NUMBER: number,
	WHAT: string,
	WHO: number,
	WHERE: string
}