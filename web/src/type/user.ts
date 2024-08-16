export type User = {
  id: string;
  email: string;
  fullName: string;
  username: string;
  givenName: string;
  familyName: string;
  picture: string;
  language: string;
  regions: string[];
  currency: string;
  createdAt: Date;
  updatedAt: Date;
};
