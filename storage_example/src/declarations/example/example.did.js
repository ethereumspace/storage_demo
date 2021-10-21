export const idlFactory = ({ IDL }) => {
  return IDL.Service({ 'mint' : IDL.Func([IDL.Nat], [], []) });
};
export const init = ({ IDL }) => { return []; };
