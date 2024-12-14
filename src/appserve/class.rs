const INPUT_STYLE: &str = "
    width: 100%;
    height: 3rem;
    background-color: #333333;
    padding-right: 1rem;
    padding-left: 1.5rem;
    padding-top: 1rem;
    padding-bottom: 1rem;
    color: white;
    margin-top: 1.5rem;
    outline: none;
    transition: all 1s ease-in-out;
    &:focus {
        outline: none;
        padding-left: 1.75rem;
    }
";

const CANCEL_BUTTON_STYLE: &str = "
    margin-top: 2.5rem;
    background-color: #555555;
    padding: 0.5rem 2rem;
    border-radius: 0.25rem;
    color: white;
    margin-right: 0.75rem;
    transition: all 1s ease-in-out;
    &:hover {
        background-color: #666666;
    }
";

const ADD_BUTTON_STYLE: &str = "
    margin-top: 2.5rem;
    background-color: #7734e7;
    padding: 0.5rem 2rem;
    border-radius: 0.25rem;
    color: white;
    transition: all 1s ease-in-out;
    &:hover {
        background-color: #8448e9;
    }
";

const NO_ERROR_STYLE: &str = "
    display: flex;
    flex-direction: column;
    background-color: #222222;
    border-top: 0.5rem solid #7734e7;
    padding-left: 1.5rem;
    padding-right: 1.5rem;
    padding-top: 1.25rem;
    height: 29rem;
    width: 100%;
    max-width: 36rem;
    z-index: 50;
    margin-top: -0.5rem;
    position: fixed;
";

const ERROR_STYLE: &str = "
    display: flex;
    flex-direction: column;
    background-color: #222222;
    border-top: 0.5rem solid #7734e7;
    padding-left: 1.5rem;
    padding-right: 1.5rem;
    padding-top: 1.25rem;
    height: 32rem;
    width: 100%;
    max-width: 36rem;
    z-index: 50;
    margin-top: -0.5rem;
    position: fixed;
";
